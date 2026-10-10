use crate::types::{ Text, Key, NotificationType };
#[cfg(target_os = "windows")]
use crate::platform::windows::notify;
#[cfg(unix)]
use crate::platform::unix::notify;

/// A text label positioned on the terminal grid.
///
/// A `Label` stores an absolute `(x, y)` position plus the foreground and
/// background colours (ANSI escape sequences from [`crate::colors`]) and the
/// [`Text`] to display.
pub struct Label {
    /// Position as `(column, row)`, using 0-based terminal cells.
    pub pos: (u16, u16),
    /// Foreground colour escape sequence.
    pub fore_color: &'static str,
    /// Background colour escape sequence.
    pub back_color: &'static str,
    /// The text to display.
    pub text: Text,
}

impl Label {
    /// Converts this label into an [`NCLabel`], dropping its position while
    /// keeping its colours and text.
    pub fn to_nc(&self) -> NCLabel {
        NCLabel {
            fore_color: self.fore_color,
            back_color: self.back_color,
            text: self.text.clone(),
        }
    }
}

/// A label without a position ("No Coordinate Label").
///
/// `NCLabel` is useful for text whose position is decided later, for example
/// when printing at the current cursor position with `Terminal::print`.
pub struct NCLabel {
    /// Foreground colour escape sequence.
    pub fore_color: &'static str,
    /// Background colour escape sequence.
    pub back_color: &'static str,
    /// The text to display.
    pub text: Text,
}

impl NCLabel {
    /// Converts this label into a positioned [`Label`] using `pos_` as its
    /// `(column, row)` position.
    pub fn to_label(&self, pos_: (u16, u16)) -> Label {
        Label {
            pos: pos_,
            fore_color: self.fore_color,
            back_color: self.back_color,
            text: self.text.clone(),
        }
    }
}

/// A single terminal cell holding one character and its colours.
///
/// Unlike [`Pixel`], a `Cell` displays a visible character in addition to a
/// background colour.
pub struct Cell {
    /// Position as `(column, row)`, using 0-based terminal cells.
    pub pos: (u16, u16),
    /// Background colour escape sequence.
    pub color: &'static str,
    /// The character to draw.
    pub char_: char,
    /// Foreground colour escape sequence for `char_`.
    pub char_color: &'static str,
}

/// A single coloured terminal cell drawn with a full block character.
pub struct Pixel {
    /// Position as `(column, row)`, using 0-based terminal cells.
    pub pos: (u16, u16),
    /// Colour escape sequence used to fill the cell.
    pub color: &'static str,
}

/// A collection of [`Pixel`]s that can be rendered together.
pub struct PixelSet {
    /// The pixels that make up this set.
    pub pixels: Vec<Pixel>,
}

impl std::fmt::Display for PixelSet {
    /// Renders every pixel as an absolute cursor move followed by a filled
    /// block, producing a string of ANSI escape sequences.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pixel in &self.pixels {
            write!(
                f,
                "\x1b[{};{}H{}█\x1b[0m",
                pixel.pos.1,
                pixel.pos.0,
                pixel.color
            )?;
        }

        Ok(())
    }
}

/// Associates a [`Key`] with a function to run when that key is pressed.
///
/// Register bindings with `Terminal::bind` and dispatch them with
/// `Terminal::process_binds`.
pub struct BIND {
    /// The function to call when `key` is pressed.
    pub func: fn (),
    /// The key that triggers `func`.
    pub key:  Key
}

/// A desktop notification with a title, body and severity.
pub struct Notification {
    /// Notification title.
    title: String,
    /// Notification body text.
    body: String,
    /// Notification severity.
    type_: NotificationType
}

impl Notification {
    /// Creates a new notification from a title, body and severity.
    ///
    /// `title` and `body` accept anything convertible into a `String`.
    pub fn new(title: impl Into<String>, body: impl Into<String>, type_: NotificationType) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            type_,
        }
    }

    /// Sends the notification using the platform's native mechanism.
    ///
    /// On Windows this shows a balloon tip via PowerShell; on Unix it uses
    /// `notify-send`.
    pub fn push(&self) {
        notify(self.title.as_str(), self.body.as_str(), self.type_);
    }
}
