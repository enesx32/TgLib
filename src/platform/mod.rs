// Platform-specific backends. Exactly one of these modules is compiled,
// depending on the target operating system.

/// Windows implementation (console sizing, key state, Ctrl+C handler and
/// balloon-tip notifications).
#[cfg(target_os = "windows")]
pub mod windows;

/// Unix implementation (termios/raw input, `ioctl` sizing, signals and
/// `notify-send` notifications).
#[cfg(unix)]
pub mod unix;
