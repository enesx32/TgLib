#![allow(dead_code)]

// ANSI escape sequences used for colours and text attributes.
//
// Each constant is a plain `&'static str` containing the escape sequence,
// so it can be used directly as a `fore_color`/`back_color` on any component
// or written to stdout. The `NONE` constant is an empty string and can be used
// when no colour should be applied.

/// Resets all colours and attributes back to the terminal defaults.
pub const RESET: &str = "\x1b[0m";

/// No colour/attribute at all (an empty string).
pub const NONE: &str = "";
/// Bold text attribute.
pub const BOLD: &str = "\x1b[1m";
/// Italic text attribute.
pub const ITALIC: &str = "\x1b[3m";

/// Standard black foreground.
pub const BLACK: &str = "\x1b[30m";
/// Standard red foreground.
pub const RED: &str = "\x1b[31m";
/// Standard green foreground.
pub const GREEN: &str = "\x1b[32m";
/// Standard yellow foreground.
pub const YELLOW: &str = "\x1b[33m";
/// Standard blue foreground.
pub const BLUE: &str = "\x1b[34m";
/// Standard magenta foreground.
pub const MAGENTA: &str = "\x1b[35m";
/// Standard cyan foreground.
pub const CYAN: &str = "\x1b[36m";
/// Standard white foreground.
pub const WHITE: &str = "\x1b[37m";

/// Bold black foreground.
pub const B_BLACK: &str = "\x1b[1;30m";
/// Bold red foreground.
pub const B_RED: &str = "\x1b[1;31m";
/// Bold green foreground.
pub const B_GREEN: &str = "\x1b[1;32m";
/// Bold yellow foreground.
pub const B_YELLOW: &str = "\x1b[1;33m";
/// Bold blue foreground.
pub const B_BLUE: &str = "\x1b[1;34m";
/// Bold magenta foreground.
pub const B_MAGENTA: &str = "\x1b[1;35m";
/// Bold cyan foreground.
pub const B_CYAN: &str = "\x1b[1;36m";
/// Bold white foreground.
pub const B_WHITE: &str = "\x1b[1;37m";

/// Italic black foreground.
pub const I_BLACK: &str = "\x1b[3;30m";
/// Italic red foreground.
pub const I_RED: &str = "\x1b[3;31m";
/// Italic green foreground.
pub const I_GREEN: &str = "\x1b[3;32m";
/// Italic yellow foreground.
pub const I_YELLOW: &str = "\x1b[3;33m";
/// Italic blue foreground.
pub const I_BLUE: &str = "\x1b[3;34m";
/// Italic magenta foreground.
pub const I_MAGENTA: &str = "\x1b[3;35m";
/// Italic cyan foreground.
pub const I_CYAN: &str = "\x1b[3;36m";
/// Italic white foreground.
pub const I_WHITE: &str = "\x1b[3;37m";

/// Standard black background.
pub const BG_BLACK: &str = "\x1b[40m";
/// Standard red background.
pub const BG_RED: &str = "\x1b[41m";
/// Standard green background.
pub const BG_GREEN: &str = "\x1b[42m";
/// Standard yellow background.
pub const BG_YELLOW: &str = "\x1b[43m";
/// Standard blue background.
pub const BG_BLUE: &str = "\x1b[44m";
/// Standard magenta background.
pub const BG_MAGENTA: &str = "\x1b[45m";
/// Standard cyan background.
pub const BG_CYAN: &str = "\x1b[46m";
/// Standard white background.
pub const BG_WHITE: &str = "\x1b[47m";

/// Bold black background.
pub const BG_B_BLACK: &str = "\x1b[1;40m";
/// Bold red background.
pub const BG_B_RED: &str = "\x1b[1;41m";
/// Bold green background.
pub const BG_B_GREEN: &str = "\x1b[1;42m";
/// Bold yellow background.
pub const BG_B_YELLOW: &str = "\x1b[1;43m";
/// Bold blue background.
pub const BG_B_BLUE: &str = "\x1b[1;44m";
/// Bold magenta background.
pub const BG_B_MAGENTA: &str = "\x1b[1;45m";
/// Bold cyan background.
pub const BG_B_CYAN: &str = "\x1b[1;46m";
/// Bold white background.
pub const BG_B_WHITE: &str = "\x1b[1;47m";

/// Italic black background.
pub const BG_I_BLACK: &str = "\x1b[3;40m";
/// Italic red background.
pub const BG_I_RED: &str = "\x1b[3;41m";
/// Italic green background.
pub const BG_I_GREEN: &str = "\x1b[3;42m";
/// Italic yellow background.
pub const BG_I_YELLOW: &str = "\x1b[3;43m";
/// Italic blue background.
pub const BG_I_BLUE: &str = "\x1b[3;44m";
/// Italic magenta background.
pub const BG_I_MAGENTA: &str = "\x1b[3;45m";
/// Italic cyan background.
pub const BG_I_CYAN: &str = "\x1b[3;46m";
/// Italic white background.
pub const BG_I_WHITE: &str = "\x1b[3;47m";
