// Unix implementation of the platform layer. Provides terminal sizing via
// `ioctl`, raw (non-blocking) keyboard input through termios, the SIGINT
// handler and `notify-send` desktop notifications.

use libc::{
    ioctl,
    winsize,
    STDIN_FILENO,
    STDOUT_FILENO,
    TIOCGWINSZ,
};

use crate::types::{ Key, NotificationType };

use std::io::{self, Write};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::OnceLock;

// Tracks whether SIGINT has been received since the handler was installed.
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

// The termios settings captured before switching to raw mode, used to restore
// the terminal in `restore_keyboard`.
static ORIGINAL_TERMIOS: OnceLock<libc::termios> = OnceLock::new();

// The file-descriptor flags captured before enabling non-blocking reads.
static ORIGINAL_FLAGS: AtomicI32 = AtomicI32::new(0);

// Flag set from the SIGINT handler; read by `is_interrupted`.
unsafe extern "C" fn ctrl_c_handler(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

/// Installs a SIGINT handler that flags the terminal as interrupted.
pub fn install_ctrl_c_handler() {
    unsafe {
        libc::signal(
            libc::SIGINT,
            ctrl_c_handler as libc::sighandler_t
        );
    }
}

/// Returns `true` if SIGINT has been received since the handler was installed.
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

/// Switches stdin to raw, non-blocking mode so that keys can be read without
/// waiting for a newline.
///
/// The previous termios settings and file-descriptor flags are saved so that
/// [`restore_keyboard`] can undo this.
pub fn setup_keyboard() {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();

        if libc::tcgetattr(STDIN_FILENO, &mut termios) != 0 {
            return;
        }

        let _ = ORIGINAL_TERMIOS.set(termios);

        let mut raw = termios;

        raw.c_lflag &= !libc::ICANON;
        raw.c_lflag &= !libc::ECHO;

        raw.c_cc[libc::VMIN] = 0;
        raw.c_cc[libc::VTIME] = 0;

        libc::tcsetattr(
            STDIN_FILENO,
            libc::TCSANOW,
            &raw
        );

        let flags = libc::fcntl(
            STDIN_FILENO,
            libc::F_GETFL
        );

        ORIGINAL_FLAGS.store(flags, Ordering::SeqCst);

        libc::fcntl(
            STDIN_FILENO,
            libc::F_SETFL,
            flags | libc::O_NONBLOCK
        );
    }
}

/// Restores the terminal and stdin to the state captured before
/// [`setup_keyboard`] was called.
pub fn restore_keyboard() {
    unsafe {
        if let Some(termios) = ORIGINAL_TERMIOS.get() {
            libc::tcsetattr(
                STDIN_FILENO,
                libc::TCSANOW,
                termios
            );
        }

        let flags = ORIGINAL_FLAGS.load(Ordering::SeqCst);

        libc::fcntl(
            STDIN_FILENO,
            libc::F_SETFL,
            flags
        );
    }
}

// Maps a single UTF-8 byte (a plain ASCII character) to the corresponding
// `Key`, or `None` if it is not recognised.
fn char_to_key(c: u8) -> Option<Key> {
    match c {
        b'a' | b'A' => Some(Key::A),
        b'b' | b'B' => Some(Key::B),
        b'c' | b'C' => Some(Key::C),
        b'd' | b'D' => Some(Key::D),
        b'e' | b'E' => Some(Key::E),
        b'f' | b'F' => Some(Key::F),
        b'g' | b'G' => Some(Key::G),
        b'h' | b'H' => Some(Key::H),
        b'i' | b'I' => Some(Key::I),
        b'j' | b'J' => Some(Key::J),
        b'k' | b'K' => Some(Key::K),
        b'l' | b'L' => Some(Key::L),
        b'm' | b'M' => Some(Key::M),
        b'n' | b'N' => Some(Key::N),
        b'o' | b'O' => Some(Key::O),
        b'p' | b'P' => Some(Key::P),
        b'q' | b'Q' => Some(Key::Q),
        b'r' | b'R' => Some(Key::R),
        b's' | b'S' => Some(Key::S),
        b't' | b'T' => Some(Key::T),
        b'u' | b'U' => Some(Key::U),
        b'v' | b'V' => Some(Key::V),
        b'w' | b'W' => Some(Key::W),
        b'x' | b'X' => Some(Key::X),
        b'y' | b'Y' => Some(Key::Y),
        b'z' | b'Z' => Some(Key::Z),

        b'1' => Some(Key::Key1),
        b'2' => Some(Key::Key2),
        b'3' => Some(Key::Key3),
        b'4' => Some(Key::Key4),
        b'5' => Some(Key::Key5),
        b'6' => Some(Key::Key6),
        b'7' => Some(Key::Key7),
        b'8' => Some(Key::Key8),
        b'9' => Some(Key::Key9),
        b'0' => Some(Key::Key0),

        b' ' => Some(Key::SPACE),
        b'\t' => Some(Key::TAB),
        b'\r' | b'\n' => Some(Key::ENTER),

        _ => None,
    }
}

/// Reads one key from stdin without blocking.
///
/// ANSI escape sequences (e.g. arrow keys) are decoded as well as plain
/// characters. Returns `None` when nothing is available.
pub fn read_key() -> Option<Key> {
    let mut buffer = [0u8; 8];

    let amount = unsafe {
        libc::read(
            STDIN_FILENO,
            buffer.as_mut_ptr() as *mut libc::c_void,
            buffer.len()
        )
    };

    if amount <= 0 {
        return None;
    }

    let amount = amount as usize;

    // A lone Escape byte (when no escape sequence follows) is the Esc key.
    if amount == 1 && buffer[0] == 27 {
        return Some(Key::ESC);
    }

    if amount >= 3
        && buffer[0] == 27
        && buffer[1] == b'['
    {
        return match buffer[2] {
            b'A' => Some(Key::UP),
            b'B' => Some(Key::DOWN),
            b'C' => Some(Key::RIGHT),
            b'D' => Some(Key::LEFT),

            _ => None,
        };
    }

    char_to_key(buffer[0])
}

/// Shows a desktop notification using `notify-send`.
///
/// The urgency level is derived from `msg_type`. Failures to launch
/// `notify-send` are silently ignored.
pub fn notify(title: &str, body: &str, msg_type: NotificationType) {
    let urgency = match msg_type {
        NotificationType::Info => "low",
        NotificationType::Warn => "normal",
        NotificationType::Error => "critical",
    };

    let _ = Command::new("notify-send")
        .arg("-u")
        .arg(urgency)
        .arg(title)
        .arg(body)
        .status();
}

/// Returns the terminal size as `(columns, rows)` using `ioctl`, falling
/// back to `(80, 24)` when the size cannot be queried.
pub fn get_terminal_size() -> (u16, u16) {
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let result = unsafe {
        ioctl(
            STDOUT_FILENO,
            TIOCGWINSZ,
            &mut size
        )
    };

    if result == 0
        && size.ws_col > 0
        && size.ws_row > 0
    {
        (size.ws_col, size.ws_row)
    } else {
        (80, 24)
    }
}

/// Returns the terminal width in columns.
pub fn get_terminal_width() -> u16 {
    get_terminal_size().0
}

/// Returns the terminal height in rows.
pub fn get_terminal_height() -> u16 {
    get_terminal_size().1
}

/// Returns the current cursor position as `(column, row)`, using 0-based cells
/// relative to the top-left of the visible screen.
///
/// The position is obtained by asking the terminal for a Device Status Report
/// (`ESC [ 6 n`) and reading its reply. Falls back to `(0, 0)` when the
/// terminal does not answer promptly.
pub fn get_cursor_position() -> (u16, u16) {
    let mut stdout = io::stdout();

    let _ = stdout.write_all(b"\x1b[6n");
    let _ = stdout.flush();

    let mut buffer = [0u8; 32];
    let mut length = 0usize;
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(50);

    while length < buffer.len() {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        let timeout = remaining.as_millis().min(i32::MAX as u128) as i32;

        let mut descriptor = libc::pollfd {
            fd: STDIN_FILENO,
            events: libc::POLLIN,
            revents: 0,
        };

        if unsafe { libc::poll(&mut descriptor, 1, timeout) } <= 0 {
            break;
        }

        let amount = unsafe {
            libc::read(
                STDIN_FILENO,
                buffer[length..].as_mut_ptr() as *mut libc::c_void,
                buffer.len() - length
            )
        };

        if amount <= 0 {
            break;
        }

        length += amount as usize;

        if buffer[..length].contains(&b'R') {
            break;
        }
    }

    parse_cursor_reply(&buffer[..length]).unwrap_or((0, 0))
}

// Parses a cursor position reply of the form `ESC [ row ; column R`, returning
// the 0-based `(column, row)` it encodes.
fn parse_cursor_reply(buffer: &[u8]) -> Option<(u16, u16)> {
    let text = std::str::from_utf8(buffer).ok()?;
    let start = text.find('[')? + 1;
    let end = text.find('R')?;

    let mut parts = text[start..end].split(';');

    let row = parts.next()?.parse::<u16>().ok()?;
    let column = parts.next()?.parse::<u16>().ok()?;

    Some((column.saturating_sub(1), row.saturating_sub(1)))
}

/// Clears the terminal by running the `clear` command.
pub fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("failed to execute process");
}