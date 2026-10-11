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

/// The main entry point for interacting with the terminal.
///
/// A `Terminal` detects its size, sets up the alternate screen buffer and
/// keyboard handling, exposes cursor control and key bindings, and provides
/// both an immediate-mode API (`printc`, `show_cell`, ...) and a flicker-free
/// buffered API (`begin_frame`, `draw`, `render`).
///
/// Most methods take `&self` because the renderer uses interior mutability,
/// so a `Terminal` can usually be used without declaring it `mut`. Only
/// [`bind`](Terminal::bind) and [`handle_key`](Terminal::handle_key) need a
/// mutable reference.
pub struct Terminal {
    /// Terminal width captured when the `Terminal` was created.
    width: u16,
    /// Terminal height captured when the `Terminal` was created.
    height: u16,
    /// Registered key bindings, dispatched by
    /// [`process_binds`](Terminal::process_binds).
    pub binds: Vec<BIND>,
    /// Double-buffered renderer backing the frame API.
    renderer: RefCell<Renderer>,
}

impl Terminal {
    /// Creates a new `Terminal` and queries the current terminal size.
    ///
    /// This does not change the terminal state. Call [`Terminal::setup`] (or
    /// [`Terminal::setup_guard`]) before drawing.
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

    /// Sleeps for `duration`, blocking the current thread.
    pub fn sleep(&self, duration: std::time::Duration) {
        std::thread::sleep(duration);
    }

    /// Registers a key binding so that its function runs when the bound key is
    /// pressed.
    pub fn bind(&mut self, bind: BIND) {
        self.binds.push(bind);
    }

    /// Runs every registered binding whose key matches `key`.
    pub fn handle_key(&self, key: Key) {
        for bind in &self.binds {
            if bind.key == key {
                (bind.func)();
            }
        }
    }

    /// Reads a single pending key (if any) and dispatches matching bindings.
    pub fn process_binds(&self) {
        if let Some(key) = read_key() {
            self.handle_key(key);
        }
    }

    // Switches to the alternate screen buffer and clears it.
    fn enter(&self) {
        print!("\x1b[?1049h");
        print!("\x1b[2J");
        print!("\x1b[H");

        io::stdout().flush().unwrap();
    }

    /// Restores the terminal to its original state.
    ///
    /// This restores keyboard settings, shows the cursor and leaves the
    /// alternate screen buffer. The buffered frame is forgotten so that a
    /// later render redraws everything.
    pub fn leave(&self) {
        restore_keyboard();
        self.renderer.borrow_mut().invalidate();

        print!("\x1b[?25h");
        print!("\x1b[?1049l");

        io::stdout().flush().unwrap();
    }

    /// Clears the screen using the platform's clear command.
    ///
    /// The buffered frame is forgotten so the next [`render`](Terminal::render)
    /// redraws all content.
    pub fn clear_screen(&self) {
        clear_screen();
        self.renderer.borrow_mut().invalidate();
        io::stdout().flush().unwrap();
    }

    // Installs the platform-specific Ctrl+C handler.
    fn install_ctrl_c_handler(&self) {
        install_ctrl_c_handler();
    }

    /// Prepares the terminal for use.
    ///
    /// Enters the alternate screen buffer, installs the Ctrl+C handler and
    /// switches keyboard input to a raw, non-blocking mode. Pair with
    /// [`Terminal::leave`] or use [`Terminal::setup_guard`] for automatic
    /// cleanup.
    pub fn setup(&self) {
        self.enter();
        self.install_ctrl_c_handler();
        setup_keyboard();
    }

    /// Returns `true` once an interrupt (Ctrl+C) has been received.
    pub fn is_interrupted(&self) -> bool {
        is_interrupted()
    }

    /// Returns the terminal width captured when the `Terminal` was created.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the terminal height captured when the `Terminal` was created.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns the current cursor position as `(column, row)` in terminal
    /// cells, relative to the top-left of the visible screen.
    ///
    /// Falls back to `(0, 0)` when the position cannot be queried.
    pub fn cursor_position(&self) -> (u16, u16) {
        get_cursor_position()
    }

    /// Draws both coordinate rulers.
    ///
    /// Equivalent to calling [`debug_coordinates_x`](Terminal::debug_coordinates_x)
    /// followed by [`debug_coordinates_y`](Terminal::debug_coordinates_y).
    pub fn debug_coordinates(&self) {
        self.debug_coordinates_x();
        self.debug_coordinates_y();
    }

    /// Prints a vertical coordinate ruler down the first column.
    ///
    /// Useful for finding the `y` value to pass to a component's position.
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

    /// Prints a horizontal coordinate ruler along the first row.
    ///
    /// Useful for finding the `x` value to pass to a component's position.
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

    /// Writes an [`NCLabel`] at the current cursor position.
    ///
    /// Applies the label's colours, prints its text and resets styles,
    /// then flushes stdout.
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

    /// Writes a [`Label`] at its absolute position and flushes immediately.
    ///
    /// This is the immediate-mode drawing API. For flicker-free, in-place
    /// updates prefer the buffered API: [`begin_frame`](Terminal::begin_frame),
    /// [`draw`](Terminal::draw) and [`render`](Terminal::render).
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

    /// Starts a new buffered frame, blanking the back buffer.
    ///
    /// Components drawn after this call (with [`draw`](Terminal::draw),
    /// [`draw_text`](Terminal::draw_text), [`draw_cell`](Terminal::draw_cell)
    /// or [`draw_pixelset`](Terminal::draw_pixelset)) are collected in memory
    /// and only written to the terminal by [`render`](Terminal::render).
    ///
    /// This also picks up terminal resizes.
    pub fn begin_frame(&self) {
        let mut renderer = self.renderer.borrow_mut();

        renderer.resize(
            get_terminal_width(),
            get_terminal_height()
        );

        renderer.begin_frame();
    }

    /// Draws a [`Label`] at its own position into the frame currently being
    /// built.
    pub fn draw(&self, label: &Label) {
        self.renderer
            .borrow_mut()
            .draw_label(label);
    }

    /// Draws text at `pos` into the frame currently being built.
    ///
    /// `fore_color` and `back_color` are escape sequences from
    /// [`crate::colors`]; anything outside the screen is clipped.
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

    /// Draws a [`Cell`](crate::components::Cell) into the frame currently
    /// being built.
    pub fn draw_cell(
        &self,
        cell: &crate::components::Cell
    ) {
        self.renderer
            .borrow_mut()
            .draw_cell(cell);
    }

    /// Draws a [`PixelSet`](crate::components::PixelSet) into the frame
    /// currently being built.
    pub fn draw_pixelset(
        &self,
        pixel_set: &crate::components::PixelSet
    ) {
        self.renderer
            .borrow_mut()
            .draw_pixelset(pixel_set);
    }

    /// Writes the frame built since [`begin_frame`](Terminal::begin_frame) to
    /// stdout, touching only the cells that changed since the previous frame,
    /// then flushes once.
    ///
    /// If nothing changed, no bytes are written at all. Write errors are
    /// ignored rather than panicking.
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

    /// Makes the cursor invisible.
    pub fn hide_cursor(&self) {
        print!("\x1b[?25l");

        std::io::stdout()
            .flush()
            .unwrap();
    }

    /// Makes the cursor visible again.
    pub fn show_cursor(&self) {
        print!("\x1b[?25h");

        std::io::stdout()
            .flush()
            .unwrap();
    }

    /// Draws a [`Cell`](crate::components::Cell) immediately at its position
    /// and flushes.
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

    /// Draws a [`Pixel`](crate::components::Pixel) immediately at its position
    /// and flushes.
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

    /// Draws every pixel of a [`PixelSet`](crate::components::PixelSet)
    /// immediately and flushes.
    pub fn show_pixelset(
        &self,
        pixel_set: &crate::components::PixelSet
    ) {
        print!("{}", pixel_set);

        io::stdout().flush().unwrap();
    }

    /// Sets up the terminal and returns a guard that restores it when dropped.
    ///
    /// Prefer this over calling [`setup`](Terminal::setup) and
    /// [`leave`](Terminal::leave) manually so that the terminal is restored on
    /// normal exit and on panics that unwind. Note that terminal state cannot
    /// be restored after a crash or a forced process termination.
    pub fn setup_guard(&self) -> TerminalGuard<'_> {
        self.setup();

        TerminalGuard {
            terminal: self,
            active: true,
        }
    }
}

/// An RAII guard that restores the terminal state when dropped.
///
/// Returned by [`Terminal::setup_guard`]. Its `Drop` implementation calls
/// [`Terminal::leave`] unless it was disarmed.
pub struct TerminalGuard<'a> {
    /// The terminal this guard is responsible for.
    terminal: &'a Terminal,
    /// Whether `leave` should still be called on drop.
    active: bool,
}

impl TerminalGuard<'_> {
    /// Prevents the guard from restoring the terminal on drop.
    ///
    /// Use this if you have already called [`Terminal::leave`] yourself and do
    /// not want it called a second time.
    pub fn disarm(&mut self) {
        self.active = false;
    }
}

impl Drop for TerminalGuard<'_> {
    /// Restores the terminal unless the guard was disarmed.
    fn drop(&mut self) {
        if self.active {
            self.terminal.leave();
        }
    }
}
