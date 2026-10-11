/// A piece of text together with its cached byte length.
///
/// `Text` is used by [`crate::components::Label`] and
/// [`crate::components::NCLabel`] to store the string that should be drawn.
#[derive(Clone)]
pub struct Text {
    /// Cached length of `val` in bytes (truncated to `u8`).
    len: u8,
    /// The underlying string value.
    val: String,
}

impl Text {
    /// Creates a `Text` from a string slice, storing the given value and
    /// caching its byte length.
    pub fn from_str(val: &str) -> Self {
        let len = val.len() as u8;
        Text { len, val: val.to_string() }
    }

    /// Returns the cached byte length of the text.
    pub fn get_len(&self) -> u8 {
        self.len
    }

    /// Returns the text as a string slice.
    pub fn get_val(&self) -> &str {
        &self.val
    }

    /// Returns an independent copy of this `Text`.
    ///
    /// This is an inherent method that mirrors the derived `Clone`
    /// implementation.
    pub fn clone(&self) -> Self {
        Text { len: self.len, val: self.val.clone() }
    }
}

/// A keyboard key recognised by the library.
///
/// Values are produced by the platform layer and matched against
/// [`crate::components::BIND`] entries in `Terminal::process_binds`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// The `A` key.
    A,
    /// The `B` key.
    B,
    /// The `C` key.
    C,
    /// The `D` key.
    D,
    /// The `E` key.
    E,
    /// The `F` key.
    F,
    /// The `G` key.
    G,
    /// The `H` key.
    H,
    /// The `I` key.
    I,
    /// The `J` key.
    J,
    /// The `K` key.
    K,
    /// The `L` key.
    L,
    /// The `M` key.
    M,
    /// The `N` key.
    N,
    /// The `O` key.
    O,
    /// The `P` key.
    P,
    /// The `Q` key.
    Q,
    /// The `R` key.
    R,
    /// The `S` key.
    S,
    /// The `T` key.
    T,
    /// The `U` key.
    U,
    /// The `V` key.
    V,
    /// The `W` key.
    W,
    /// The `X` key.
    X,
    /// The `Y` key.
    Y,
    /// The `Z` key.
    Z,
    /// The `1` key.
    Key1,
    /// The `2` key.
    Key2,
    /// The `3` key.
    Key3,
    /// The `4` key.
    Key4,
    /// The `5` key.
    Key5,
    /// The `6` key.
    Key6,
    /// The `7` key.
    Key7,
    /// The `8` key.
    Key8,
    /// The `9` key.
    Key9,
    /// The `0` key.
    Key0,
    /// The `F1` function key.
    F1,
    /// The `F2` function key.
    F2,
    /// The `F3` function key.
    F3,
    /// The `F4` function key.
    F4,
    /// The `F5` function key.
    F5,
    /// The `F6` function key.
    F6,
    /// The `F7` function key.
    F7,
    /// The `F8` function key.
    F8,
    /// The `F9` function key.
    F9,
    /// The `F10` function key.
    F10,
    /// The `F11` function key.
    F11,
    /// The `F12` function key.
    F12,
    /// The `Tab` key.
    TAB,
    /// The `Caps Lock` key.
    CAPS,
    /// The `Shift` key.
    SHIFT,
    /// The `Ctrl` key.
    CTRL,
    /// The `Windows`/`Super` key.
    WIN,
    /// The `Alt` key.
    ALT,
    /// The `Space` key.
    SPACE,
    /// The `Enter`/`Return` key.
    ENTER,
    /// The `Escape` key.
    ESC,
    /// The `Up` arrow key.
    UP,
    /// The `Down` arrow key.
    DOWN,
    /// The `Left` arrow key.
    LEFT,
    /// The `Right` arrow key.
    RIGHT,
}

/// The severity of a desktop [`crate::components::Notification`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    /// An informational notification.
    Info,
    /// A warning notification.
    Warn,
    /// An error notification.
    Error,
}
