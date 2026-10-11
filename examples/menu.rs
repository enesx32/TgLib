// Interactive menu example, in the style of `ollama launch opencode`.
//
// Use the UP/DOWN arrow keys to move the selection marker, ENTER to confirm
// the highlighted option and ESC (or Ctrl+C) to exit.
//
// The menu is drawn in the normal terminal buffer, starting at the cursor's
// current row so it appears below existing output. It never enters the
// alternate screen and never clears the screen, so output that was already
// on screen stays visible and remains in the scrollback.
//
// Run with: cargo run --example menu

use tglib::Terminal;
use tglib::components::Label;
use tglib::components::BIND;
use tglib::colors::*;
use tglib::types::Key;
use tglib::types::Text;

#[cfg(target_os = "windows")]
use tglib::platform::windows::{install_ctrl_c_handler, restore_keyboard, setup_keyboard};

#[cfg(unix)]
use tglib::platform::unix::{install_ctrl_c_handler, restore_keyboard, setup_keyboard};

use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

// The menu options, each with a one-line description.
const ITEMS: &[(&str, &str)] = &[
    ("Launch Counter", "Opens the buffered rendering demo"),
    ("Launch Settings", "Opens the settings panel"),
    ("About TgLib", "Shows version and feature info"),
    ("Exit", "Leaves the menu"),
];

// Fixed layout coordinates (column, row), using 0-based terminal cells. Rows
// are relative to `top`, the starting row captured before drawing, so the menu
// is placed below existing output instead of at the top of the screen.
const ITEM_TOP: u16 = 2;
const ITEM_GAP: u16 = 3;
const STATUS_ROW: u16 = 16;

// Total number of rows the menu occupies, from its header down to the status
// line. Used to make sure the whole menu fits on screen before it is drawn.
const MENU_HEIGHT: u16 = STATUS_ROW + 1;

// Row used for the confirmation message once the menu has closed. It sits
// below the menu so nothing is overwritten and nothing has to be cleared.
const CONFIRM_ROW: u16 = 18;

// Which menu option is currently highlighted.
static SELECTED: AtomicUsize = AtomicUsize::new(0);

// Whether the menu is still running. Set to false by the quit handler.
static RUNNING: AtomicBool = AtomicBool::new(true);

// Hold the index of the confirmed option, or `usize::MAX` if the menu was
// left with ESC. Read once the terminal has been restored.
static ACTIVATED: AtomicUsize = AtomicUsize::new(usize::MAX);

// Restores the terminal input mode and shows the cursor when dropped.
//
// This performs the cleanup half of `Terminal::setup_guard` (pairing
// `setup_keyboard` with `restore_keyboard`) without entering or leaving the
// alternate screen, so the normal terminal buffer is left intact.
struct TerminalCleanup<'a>(&'a Terminal);

impl Drop for TerminalCleanup<'_> {
    fn drop(&mut self) {
        restore_keyboard();
        self.0.show_cursor();
    }
}

// Moves the selection by `direction` (+1 or -1), wrapping at the ends.
fn move_selection(direction: isize) {
    let len = ITEMS.len() as isize;
    let current = SELECTED.load(Ordering::Relaxed) as isize;
    let next = (current + direction + len) % len;
    SELECTED.store(next as usize, Ordering::Relaxed);
}

fn move_up() {
    move_selection(-1);
}

fn move_down() {
    move_selection(1);
}

fn activate() {
    let index = SELECTED.load(Ordering::Relaxed);
    if ITEMS[index].0 == "Exit" {
        RUNNING.store(false, Ordering::Relaxed);
    } else {
        ACTIVATED.store(index, Ordering::Relaxed);
        RUNNING.store(false, Ordering::Relaxed);
    }
}

fn quit() {
    RUNNING.store(false, Ordering::Relaxed);
}

fn main() {
    // Bindings are registered before the cleanup guard because `bind` needs a
    // mutable reference while the guard only borrows immutably.
    let mut terminal = Terminal::new();
    terminal.bind(BIND { func: move_up, key: Key::UP });
    terminal.bind(BIND { func: move_down, key: Key::DOWN });
    terminal.bind(BIND { func: activate, key: Key::ENTER });
    terminal.bind(BIND { func: quit, key: Key::ESC });

    // Switch to raw, non-blocking input and install the Ctrl+C handler.
    // Unlike `Terminal::setup`, this does not touch the alternate screen.
    setup_keyboard();
    install_ctrl_c_handler();

    // Move onto a fresh line just below the existing output (for example
    // Cargo's "Running ..." line) and capture that row. Every menu coordinate
    // is drawn relative to it, so nothing already on screen is overwritten and
    // the renderer never resets drawing to row 0.
    print!("\r\n");
    let _ = std::io::stdout().flush();
    let (_, mut top) = terminal.cursor_position();

    // If the menu would run past the bottom of the screen, scroll the terminal
    // up so it ends on the last row. Printing `MENU_HEIGHT - 1` newlines moves
    // the cursor down over the blank rows below it and scrolls the terminal by
    // exactly the amount that is missing. Drawing from `height - MENU_HEIGHT`
    // then keeps the whole menu visible, while any output that was in the way is
    // pushed into the scrollback instead of being lost or drawn off-screen.
    let height = terminal.height();
    if top + MENU_HEIGHT > height {
        for _ in 0..MENU_HEIGHT - 1 {
            println!();
        }
        let _ = std::io::stdout().flush();

        top = height.saturating_sub(MENU_HEIGHT);
    }

    let cleanup = TerminalCleanup(&terminal);
    terminal.hide_cursor();

    while RUNNING.load(Ordering::Relaxed) && !terminal.is_interrupted() {
        terminal.begin_frame();

        // Header line.
        let header = Label {
            pos: (0, top),
            fore_color: B_WHITE,
            back_color: BG_BLACK,
            text: Text::from_str("TgLib 0.2.6"),
        };
        terminal.draw(&header);

        // One row per option: selection marker, title and description.
        for (index, (title, subtitle)) in ITEMS.iter().enumerate() {
            let row = top + ITEM_TOP + index as u16 * ITEM_GAP;
            let selected = index == SELECTED.load(Ordering::Relaxed);

            let marker = Label {
                pos: (0, row),
                fore_color: if selected { GREEN } else { WHITE },
                back_color: BG_BLACK,
                text: if selected {
                    Text::from_str("▸")
                } else {
                    Text::from_str(" ")
                },
            };
            terminal.draw(&marker);

            let title_text = Label {
                pos: (2, row),
                fore_color: if selected { B_GREEN } else { WHITE },
                back_color: BG_BLACK,
                text: Text::from_str(title),
            };
            terminal.draw(&title_text);

            let subtitle_text = Label {
                pos: (4, row + 1),
                fore_color: if selected { GREEN } else { WHITE },
                back_color: BG_BLACK,
                text: Text::from_str(subtitle),
            };
            terminal.draw(&subtitle_text);
        }

        // Footer with the available keys.
        let footer = Label {
            pos: (0, top + 14),
            fore_color: WHITE,
            back_color: BG_BLACK,
            text: Text::from_str("UP/DOWN navigate · ENTER confirm · ESC quit"),
        };
        terminal.draw(&footer);

        // Status line showing the currently highlighted option.
        let status = Label {
            pos: (0, top + STATUS_ROW),
            fore_color: GREEN,
            back_color: BG_BLACK,
            text: Text::from_str(&format!(
                "Selected: {}",
                ITEMS[SELECTED.load(Ordering::Relaxed)].0
            )),
        };
        terminal.draw(&status);

        terminal.render();

        terminal.process_binds();
        terminal.sleep(std::time::Duration::from_millis(16));
    }

    // Restore the input mode and cursor before printing the confirmation, so
    // the message appears on the settled screen rather than hidden.
    drop(cleanup);

    let activated = ACTIVATED.load(Ordering::Relaxed);
    if activated != usize::MAX {
        // Move below the menu instead of clearing anything.
        print!("\x1b[{};1H", top + CONFIRM_ROW + 1);
        println!("Activated: {}", ITEMS[activated].0);
    }
}
