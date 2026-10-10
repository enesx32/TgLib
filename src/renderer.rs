// A minimal double-buffered renderer used by `Terminal` to implement
// flicker-free, in-place updates. The back buffer (`current`) is built each
// frame; `render` diffs it against the front buffer (`previous`) and emits
// only the escape sequences needed to update the changed cells.

use crate::colors::RESET;
use crate::components::{Cell, Label, NCLabel, PixelSet};

/// A single cell in the off-screen buffer.
///
/// Cells compare by value so the renderer can diff the current frame against
/// the previous one.
#[derive(Clone, PartialEq, Eq)]
struct BufferCell {
    /// The character stored in this cell.
    ch: char,
    /// Foreground colour escape sequence.
    fore: &'static str,
    /// Background colour escape sequence.
    back: &'static str,
}

impl BufferCell {
    /// Returns an empty cell: a space drawn with the terminal's default
    /// colours.
    const fn blank() -> Self {
        BufferCell {
            ch: ' ',
            fore: RESET,
            back: RESET,
        }
    }
}

/// A double-buffered grid of terminal cells.
///
/// Build a frame with [`Renderer::begin_frame`] and the `draw_*` methods,
/// then call [`Renderer::render`] to get the escape sequences that only
/// touch the cells which changed since the previous frame.
pub struct Renderer {
    /// Width of the buffer in cells.
    width: u16,
    /// Height of the buffer in cells.
    height: u16,
    /// The frame currently being built (back buffer).
    current: Vec<BufferCell>,
    /// The last frame written to the terminal (front buffer).
    previous: Vec<BufferCell>,
    /// When `true`, the next render clears the screen first (used after a
    /// resize or an external invalidation).
    needs_clear: bool,
}

impl Renderer {
    /// Creates a renderer with a back and front buffer of `width × height`
    /// blank cells.
    pub fn new(width: u16, height: u16) -> Self {
        let size = width as usize * height as usize;

        Renderer {
            width,
            height,
            current: vec![BufferCell::blank(); size],
            previous: vec![BufferCell::blank(); size],
            needs_clear: false,
        }
    }

    /// Returns the current buffer width in cells.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the current buffer height in cells.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Resizes the buffers. The next render will clear the screen so that
    /// stale contents from the old layout cannot remain visible.
    pub fn resize(&mut self, width: u16, height: u16) {
        if width == self.width && height == self.height {
            return;
        }

        self.width = width;
        self.height = height;

        let size = width as usize * height as usize;

        self.current = vec![BufferCell::blank(); size];
        self.previous = vec![BufferCell::blank(); size];
        self.needs_clear = true;
    }

    /// Forgets what was previously drawn, forcing the next render to redraw
    /// the current frame even if its contents did not change.
    pub fn invalidate(&mut self) {
        for cell in &mut self.previous {
            *cell = BufferCell::blank();
        }

        self.needs_clear = true;
    }

    /// Starts a new frame by blanking the back buffer.
    pub fn begin_frame(&mut self) {
        for cell in &mut self.current {
            *cell = BufferCell::blank();
        }
    }

    // Writes a single cell at `(x, y)` into the back buffer, ignoring
    // coordinates that fall outside the screen.
    fn set(&mut self, x: u16, y: u16, ch: char, fore: &'static str, back: &'static str) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y as usize * self.width as usize + x as usize;
        self.current[index] = BufferCell { ch, fore, back };
    }

    /// Draws a string starting at `(x, y)`, clipping anything outside the screen.
    pub fn draw_text(
        &mut self,
        x: u16,
        y: u16,
        fore: &'static str,
        back: &'static str,
        text: &str,
    ) {
        if y >= self.height {
            return;
        }

        let row = y as usize * self.width as usize;

        for (offset, ch) in text.chars().enumerate() {
            let column = x as usize + offset;

            if column >= self.width as usize {
                break;
            }

            self.current[row + column] = BufferCell { ch, fore, back };
        }
    }

    /// Draws a [`Label`] at its own position into the back buffer.
    pub fn draw_label(&mut self, label: &Label) {
        self.draw_text(
            label.pos.0,
            label.pos.1,
            label.fore_color,
            label.back_color,
            label.text.get_val(),
        );
    }

    /// Draws an [`NCLabel`] at `pos` into the back buffer.
    pub fn draw_nclabel(&mut self, label: &NCLabel, pos: (u16, u16)) {
        self.draw_text(
            pos.0,
            pos.1,
            label.fore_color,
            label.back_color,
            label.text.get_val(),
        );
    }

    /// Draws a single [`Cell`] (character plus colours) into the back buffer.
    pub fn draw_cell(&mut self, cell: &Cell) {
        self.set(
            cell.pos.0,
            cell.pos.1,
            cell.char_,
            cell.char_color,
            cell.color,
        );
    }

    /// Draws every [`Pixel`](crate::components::Pixel) in a
    /// [`PixelSet`] into the back buffer as a filled block.
    pub fn draw_pixelset(&mut self, pixel_set: &PixelSet) {
        for pixel in &pixel_set.pixels {
            self.set(pixel.pos.0, pixel.pos.1, '█', pixel.color, RESET);
        }
    }

    /// Diffs the current frame against the previous one and returns the
    /// escape sequences needed to update the screen. If nothing changed the
    /// result is an empty string.
    pub fn render(&mut self) -> String {
        let mut output = String::new();

        if self.needs_clear {
            output.push_str("\x1b[2J");
            self.needs_clear = false;
        }

        let mut last_x: i32 = -1;
        let mut last_y: i32 = -1;
        let mut active_fore = RESET;
        let mut active_back = RESET;

        for y in 0..self.height as usize {
            let row = y * self.width as usize;

            for x in 0..self.width as usize {
                let current = &self.current[row + x];

                if current == &self.previous[row + x] {
                    continue;
                }

                if last_x != x as i32 || last_y != y as i32 {
                    output.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
                }

                if current.fore != active_fore || current.back != active_back {
                    output.push_str(RESET);
                    output.push_str(current.fore);
                    output.push_str(current.back);
                    active_fore = current.fore;
                    active_back = current.back;
                }

                output.push(current.ch);
                last_x = x as i32 + 1;
                last_y = y as i32;
            }
        }

        if active_fore != RESET || active_back != RESET {
            output.push_str(RESET);
        }

        self.previous.clone_from(&self.current);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::{BG_BLACK, BG_RED, GREEN, RED};
    use crate::components::Label;
    use crate::types::Text;

    // Builds a `Label` for use in the tests below.
    fn label(x: u16, y: u16, fore: &'static str, back: &'static str, text: &str) -> Label {
        Label {
            pos: (x, y),
            fore_color: fore,
            back_color: back,
            text: Text::from_str(text),
        }
    }

    #[test]
    // A single rendered label produces its cursor move, text and colours.
    fn frame_contains_text_position_and_colors() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));

        let output = renderer.render();

        assert!(output.contains("\x1b[1;1H"));
        assert!(output.contains("Hi"));
        assert!(output.contains(GREEN));
        assert!(output.contains(BG_BLACK));
    }

    #[test]
    // Rendering the exact same frame twice emits nothing the second time.
    fn unchanged_frame_produces_no_output() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let _ = renderer.render();

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let output = renderer.render();

        assert!(output.is_empty());
    }

    #[test]
    // Shortening a label replaces the leftover characters with spaces.
    fn shorter_text_clears_leftover_characters() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hello"));
        let _ = renderer.render();

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let output = renderer.render();

        assert!(output.contains('i'));
        assert!(!output.contains('l'));
        assert!(!output.contains('o'));

        let trailing_spaces = output.matches(' ').count();
        assert!(trailing_spaces >= 3);
    }

    #[test]
    // Changing a label's colours emits the new escape sequences.
    fn changing_colors_emits_new_style() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let _ = renderer.render();

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, RED, BG_RED, "Hi"));
        let output = renderer.render();

        assert!(output.contains(RED));
        assert!(output.contains(BG_RED));
        assert!(output.contains("Hi"));
    }

    #[test]
    // Several labels at different positions are all present in one frame.
    fn draws_multiple_labels() {
        let mut renderer = Renderer::new(30, 10);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "first"));
        renderer.draw_label(&label(10, 4, RED, BG_BLACK, "second"));
        let output = renderer.render();

        assert!(output.contains("first"));
        assert!(output.contains("second"));
        assert!(output.contains("\x1b[1;1H"));
        assert!(output.contains("\x1b[5;11H"));
    }

    #[test]
    // Text that runs past the right edge is clipped.
    fn clips_text_to_screen() {
        let mut renderer = Renderer::new(5, 3);

        renderer.begin_frame();
        renderer.draw_label(&label(3, 0, GREEN, BG_BLACK, "ABCDEFGH"));
        let output = renderer.render();

        assert!(output.contains("AB"));
        assert!(!output.contains('C'));
        assert!(!output.contains('D'));
    }

    #[test]
    // Draws fully outside the screen are ignored.
    fn ignores_draws_outside_screen() {
        let mut renderer = Renderer::new(10, 4);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 10, GREEN, BG_BLACK, "gone"));
        renderer.draw_label(&label(20, 0, GREEN, BG_BLACK, "gone"));
        let output = renderer.render();

        assert!(!output.contains("gone"));
    }

    #[test]
    // `invalidate` forces an unchanged frame to be redrawn.
    fn invalidate_forces_redraw_of_unchanged_frame() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let _ = renderer.render();

        renderer.invalidate();

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let output = renderer.render();

        assert!(output.contains("Hi"));
    }

    #[test]
    // Multi-byte UTF-8 characters are drawn correctly.
    fn handles_multibyte_characters() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "héllo"));
        let output = renderer.render();

        assert!(output.contains("héllo"));
    }

    #[test]
    // Resizing clears the screen and redraws the current content.
    fn resize_clears_screen_and_redraws() {
        let mut renderer = Renderer::new(20, 5);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let _ = renderer.render();

        renderer.resize(10, 3);

        renderer.begin_frame();
        renderer.draw_label(&label(0, 0, GREEN, BG_BLACK, "Hi"));
        let output = renderer.render();

        assert!(output.contains("\x1b[2J"));
        assert!(output.contains("Hi"));
    }
}
