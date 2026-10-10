#[cfg(target_os = "windows")]
use crate::platform::windows::*;

#[cfg(unix)]
use crate::platform::unix::*;

use crate::components::{Label, NCLabel, BIND};
use crate::colors::RESET;
use crate::renderer::Renderer;
use std::cell::RefCell;
use std::io::{self, Write};
use crate::colors::*;
use crate::types::Key;

pub struct Terminal {
    width: u16,
    height: u16,
    pub binds: Vec<BIND>,
    renderer: RefCell<Renderer>,
}

impl Terminal {
    pub fn new() -> Self {
        let width = get_terminal_width();
        let height = get_terminal_height();

        Terminal {
            width,
            height,
            binds: Vec::new(),
            renderer: RefCell::new(Renderer::new(width, height)),
        }
    }

    pub fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }

    pub fn bind(&mut self, bind: BIND) {
        self.binds.push(bind);
    }

    pub fn handle_key(&self, key: Key) {
        for bind in &self.binds {
            if bind.key == key {
                (bind.func)();
            }
        }
    }

    pub fn process_binds(&self) {
        if let Some(key) = read_key() {
            self.handle_key(key);
        }
    }

    fn enter(&self) {
        print!("\x1b[?1049h");
        print!("\x1b[2J");
        print!("\x1b[H");

        io::stdout().flush().unwrap();
    }

    pub fn leave(&self) {
        restore_keyboard();
        self.renderer.borrow_mut().invalidate();

        print!("\x1b[?25h");
        print!("\x1b[?1049l");

        io::stdout().flush().unwrap();
    }

    pub fn clear_screen(&self) {
        clear_screen();
        self.renderer.borrow_mut().invalidate();
        io::stdout().flush().unwrap();
    }

    fn install_ctrl_c_handler(&self) {
        install_ctrl_c_handler();
    }

    pub fn setup(&self) {
        self.enter();
        self.install_ctrl_c_handler();
        setup_keyboard();
    }

    pub fn is_interrupted(&self) -> bool {
        is_interrupted()
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn debug_coordinates(&self) {
        self.debug_coordinates_x();
        self.debug_coordinates_y();
    }

    pub fn debug_coordinates_y(&self) {
        for y in 1..=self.height {
            if y < 10 {
                print!(
                    "\x1b[{};1H{}0{}~{}",
                    y,
                    B_CYAN,
                    y,
                    RESET
                );
            } else {
                print!(
                    "\x1b[{};1H{}{}~{}",
                    y,
                    B_CYAN,
                    y,
                    RESET
                );
            }
        }

        io::stdout().flush().unwrap();
    }

    pub fn debug_coordinates_x(&self) {
        let pattern = format!(
            "{I_CYAN}0123456789{RESET}"
        );

        let count = (self.width + 9) / 10;

        print!(
            "\x1b[1;1H{}",
            pattern.repeat(count.into())
        );

        io::stdout().flush().unwrap();
    }

    pub fn print(&self, text: &NCLabel) {
        print!(
            "{}{}{}{}",
            text.fore_color,
            text.back_color,
            text.text.get_val(),
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn printc(&self, text: &Label) {
        print!(
            "\x1b[{};{}H{}{}{}{}",
            text.pos.1,
            text.pos.0,
            text.fore_color,
            text.back_color,
            text.text.get_val(),
            RESET
        );

        io::stdout().flush().unwrap();
    }

    // Starts a new buffered frame. Components drawn after this call are
    // collected in memory and only written to the terminal by `render`.
    pub fn begin_frame(&self) {
        let mut renderer = self.renderer.borrow_mut();

        renderer.resize(
            get_terminal_width(),
            get_terminal_height()
        );

        renderer.begin_frame();
    }

    // Draws a Label into the frame currently being built.
    pub fn draw(&self, label: &Label) {
        self.renderer
            .borrow_mut()
            .draw_label(label);
    }

    // Draws text into the frame currently being built.
    pub fn draw_text(
        &self,
        pos: (u16, u16),
        fore_color: &'static str,
        back_color: &'static str,
        text: &str
    ) {
        self.renderer.borrow_mut().draw_text(
            pos.0,
            pos.1,
            fore_color,
            back_color,
            text
        );
    }

    // Draws a Cell into the frame currently being built.
    pub fn draw_cell(
        &self,
        cell: &crate::components::Cell
    ) {
        self.renderer
            .borrow_mut()
            .draw_cell(cell);
    }

    // Draws a PixelSet into the frame currently being built.
    pub fn draw_pixelset(
        &self,
        pixel_set: &crate::components::PixelSet
    ) {
        self.renderer
            .borrow_mut()
            .draw_pixelset(pixel_set);
    }

    // Writes the frame built since `begin_frame` to stdout, touching only
    // the cells that changed since the previous frame, then flushes once.
    pub fn render(&self) {
        let output = self.renderer.borrow_mut().render();

        if output.is_empty() {
            return;
        }

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        // Never panic on a failed terminal write.
        let _ = handle.write_all(output.as_bytes());
        let _ = handle.flush();
    }

    pub fn hide_cursor(&self) {
        print!("\x1b[?25l");

        std::io::stdout()
            .flush()
            .unwrap();
    }

    pub fn show_cursor(&self) {
        print!("\x1b[?25h");

        std::io::stdout()
            .flush()
            .unwrap();
    }

    pub fn show_cell(
        &self,
        cell: &crate::components::Cell
    ) {
        print!(
            "\x1b[{};{}H{}{}{}{}",
            cell.pos.1,
            cell.pos.0,
            cell.color,
            cell.char_color,
            cell.char_,
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn show_pixel(
        &self,
        pixel: &crate::components::Pixel
    ) {
        print!(
            "\x1b[{};{}H{}{}{}",
            pixel.pos.1,
            pixel.pos.0,
            pixel.color,
            ' ',
            RESET
        );

        io::stdout().flush().unwrap();
    }

    pub fn show_pixelset(
        &self,
        pixel_set: &crate::components::PixelSet
    ) {
        print!("{}", pixel_set);

        io::stdout().flush().unwrap();
    }

    // Sets up the terminal and returns a guard that restores it when dropped.
    // Prefer this over calling `setup` and `leave` manually so that the
    // terminal is restored on normal exit and on panics that unwind.
    pub fn setup_guard(&self) -> TerminalGuard<'_> {
        self.setup();

        TerminalGuard {
            terminal: self,
            active: true,
        }
    }
}

// Restores the terminal state when dropped.
pub struct TerminalGuard<'a> {
    terminal: &'a Terminal,
    active: bool,
}

impl TerminalGuard<'_> {
    // Prevents the guard from restoring the terminal on drop. Use this after
    // calling `leave` yourself.
    pub fn disarm(&mut self) {
        self.active = false;
    }
}

impl Drop for TerminalGuard<'_> {
    fn drop(&mut self) {
        if self.active {
            self.terminal.leave();
        }
    }
}