use crate::types::{ Text, Key, NotificationType };
#[cfg(target_os = "windows")]
use crate::platform::windows::notify;
#[cfg(unix)]
use crate::platform::unix::notify;

pub struct Label {
    pub pos: (u16, u16),
    pub fore_color: &'static str,
    pub back_color: &'static str,
    pub text: Text,
}

impl Label {
    pub fn to_nc(&self) -> NCLabel {
        NCLabel {
            fore_color: self.fore_color,
            back_color: self.back_color,
            text: self.text.clone(),
        }
    }
}

pub struct NCLabel {
    pub fore_color: &'static str,
    pub back_color: &'static str,
    pub text: Text,
}

impl NCLabel {
    pub fn to_label(&self, pos_: (u16, u16)) -> Label {
        Label {
            pos: pos_,
            fore_color: self.fore_color,
            back_color: self.back_color,
            text: self.text.clone(),
        }
    }
}

pub struct Cell {
    pub pos: (u16, u16),
    pub color: &'static str,
    pub char_: char,    
    pub char_color: &'static str,
}


pub struct Pixel {
    pub pos: (u16, u16),
    pub color: &'static str,
}

pub struct PixelSet {
    pub pixels: Vec<Pixel>,
}

impl std::fmt::Display for PixelSet {
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

pub struct BIND {
    pub func: fn (),
    pub key:  Key
}

pub struct Notification {
    title: String,
    body: String,
    type_: NotificationType
}

impl Notification {
    pub fn new(title: impl Into<String>, body: impl Into<String>, type_: NotificationType) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            type_,
        }
    }

    pub fn push(&self) {
        notify(self.title.as_str(), self.body.as_str(), self.type_);
    }
}