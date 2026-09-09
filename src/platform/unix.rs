use libc::{
    ioctl,
    winsize,
    STDIN_FILENO,
    STDOUT_FILENO,
    TIOCGWINSZ,
};

use crate::types::{ Key, NotificationType };

use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::OnceLock;

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

static ORIGINAL_TERMIOS: OnceLock<libc::termios> = OnceLock::new();

static ORIGINAL_FLAGS: AtomicI32 = AtomicI32::new(0);

unsafe extern "C" fn ctrl_c_handler(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

pub fn install_ctrl_c_handler() {
    unsafe {
        libc::signal(
            libc::SIGINT,
            ctrl_c_handler as libc::sighandler_t
        );
    }
}

pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

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

        _ => None,
    }
}

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

pub fn get_terminal_width() -> u16 {
    get_terminal_size().0
}

pub fn get_terminal_height() -> u16 {
    get_terminal_size().1
}

pub fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("failed to execute process");
}