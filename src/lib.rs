// TgLib (Terminal Graphics Library) is a lightweight, cross-platform terminal
// UI library for Rust. It offers terminal setup/teardown, colour constants,
// text components, pixel/cell graphics, key bindings, desktop notifications
// and a flicker-free double-buffered renderer.

/// Terminal management: setup/teardown, dimensions, cursor control, key
/// handling and the buffered rendering entry points.
pub mod terminal;

/// UI components such as `Label`, `Cell`, `Pixel`, `PixelSet` and
/// `Notification`, plus the `BIND` key-binding struct.
pub mod components;

/// ANSI escape sequences for foreground colours, background colours and text
/// attributes. Every constant is a `&'static str` that can be passed directly
/// to the components.
pub mod colors;

/// Platform-specific implementations, selected at compile time for Windows
/// and Unix.
pub mod platform;

/// The double-buffered [`renderer::Renderer`] that backs
/// [`terminal::Terminal`]'s frame API.
pub mod renderer;

/// Shared value types such as [`types::Text`], [`types::Key`] and
/// [`types::NotificationType`].
pub mod types;

/// Convenience re-export of [`terminal::Terminal`] and its RAII cleanup guard
/// [`terminal::TerminalGuard`].
pub use terminal::{ Terminal, TerminalGuard };

/// Convenience re-export of the low-level [`renderer::Renderer`].
pub use renderer::Renderer;

/// Convenience re-export of [`components::Label`].
pub use components::Label;

/// Convenience re-export of [`types::Text`].
pub use types::Text;

/// Convenience re-export of every colour and attribute constant.
pub use colors::*;
