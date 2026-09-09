#[derive(Clone)]
pub struct Text {
    len: u8,
    val: String,
}

impl Text {
    pub fn from_str(val: &str) -> Self {
        let len = val.len() as u8;
        Text { len, val: val.to_string() }
    }

    pub fn get_len(&self) -> u8 {
        self.len
    }

    pub fn get_val(&self) -> &str {
        &self.val
    }

    pub fn clone(&self) -> Self {
        Text { len: self.len, val: self.val.clone() }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    TAB, CAPS, SHIFT, CTRL, WIN, ALT, SPACE, UP, DOWN, LEFT, RIGHT
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Warn,
    Error,
}