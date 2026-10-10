# TgLib

A lightweight terminal UI library for Rust.

TgLib (Terminal Graphics Library) makes building terminal-based applications easier with simple abstractions for terminal management, text, colours, components, and key bindings.

[![Crates.io](https://img.shields.io/crates/v/tglib-rs?style=flat-square)](https://crates.io/crates/tglib-rs)
[![Downloads](https://img.shields.io/crates/d/tglib-rs?style=flat-square)](https://crates.io/crates/tglib-rs)
[![Documentation](https://img.shields.io/docsrs/tglib-rs?style=flat-square)](https://docs.rs/tglib-rs)
[![License](https://img.shields.io/crates/l/tglib-rs?style=flat-square)](https://github.com/enesx32/TgLib)
[![GitHub Stars](https://img.shields.io/github/stars/enesx32/TgLib?style=flat-square)](https://github.com/enesx32/TgLib)

## Features

- Terminal size detection
- Cross-platform terminal support
- Screen clearing and terminal management
- Foreground and background colours
- Text components and positioned labels
- Pixel-based terminal graphics
- Individual character cells
- Key binding support
- Desktop notifications
- Windows and Unix support

## Installation

Add TgLib to your Rust project using Cargo:

```powershell
cargo add tglib-rs
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
tglib-rs = "0.2.3"
```

**Requirements**
- Rust
- Cargo

Platform-specific dependencies are managed through Cargo.

## Basic Example

This example creates a terminal, displays a coloured label, and exits when the terminal is interrupted.

```rust
use tglib::Terminal;
use tglib::components::Label;
use tglib::colors::*;
use tglib::types::Text;
use std::time::Duration;

fn main() {
    let terminal = Terminal::new();

    terminal.setup();

    let my_text = Label {
        pos: (40, 3),
        fore_color: GREEN,
        back_color: BG_BLACK,
        text: Text::from_str(
            "Welcome to TgLib! | Version 0.2.3"
        ),
    };

    loop {
        terminal.printc(&my_text);
        terminal.hide_cursor();

        if terminal.is_interrupted() {
            let interrupt_text = Label {
                pos: (40, 5),
                fore_color: RED,
                back_color: BG_BLACK,
                text: Text::from_str("Exiting Program"),
            };

            terminal.printc(&interrupt_text);
            terminal.sleep(Duration::from_secs(2));
            terminal.leave();

            break;
        }

        terminal.sleep(Duration::from_millis(16));
    }
}
```

Run your application with:

```powershell
cargo run
```

## Colours

TgLib provides colour constants through its `colors` module.

```rust
use tglib::colors::*;

GREEN
RED
BLUE
WHITE
BG_BLACK
BG_RED
RESET
```

Use foreground colours to change text colour and background colours to change the colour behind text.

More colours and styles are available in the `colors` module.

## Terminal Coordinates

TgLib uses `(x, y)` coordinates to position components.

For example:

```rust
pos: (40, 3)
```

This represents column `40` and row `3`.

Coordinates use terminal character cells, not physical screen pixels.

## Pixels and Cells

TgLib provides ways to draw simple graphics directly in the terminal.

### Pixels

A pixel represents a coloured space in the terminal. Multiple pixels can be combined into a `PixelSet` to create shapes and patterns.

For example, you can arrange yellow and black pixels to draw a smiley face.

```rust
use tglib::components::{Pixel, PixelSet};
use tglib::colors::*;

let pixel_set = PixelSet {
    pixels: vec![
        Pixel { pos: (40, 5), color: YELLOW },
        Pixel { pos: (41, 5), color: YELLOW },
        Pixel { pos: (42, 5), color: YELLOW },
        Pixel { pos: (43, 5), color: YELLOW },
        Pixel { pos: (44, 5), color: YELLOW },
    ],
};

terminal.show_pixelset(&pixel_set);
```

Add more pixels to create your own designs. The example above shows a small section of a design rather than a complete smiley face.

### Cells

A cell can contain a character with its own foreground and background colours.

For example:

```rust
use tglib::components::Cell;
use tglib::colors::*;

let my_cell = Cell {
    pos: (1, 1),
    color: BG_RED,
    char_: '#',
    char_color: BLUE,
};

terminal.show_cell(my_cell);
```

Unlike a pixel, a cell can display a character.

## Key Bindings

TgLib supports binding keys to functions.

For example, you can bind the space bar to a function that increments a counter.

```rust
use tglib::components::BIND;
use tglib::types::Key;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn inc_count() {
    COUNT.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    let terminal = Terminal::new();

    terminal.setup();

    let counter_bind = BIND {
        func: inc_count,
        key: Key::SPACE,
    };

    terminal.bind(counter_bind);

    loop {
        terminal.process_binds();

        let count = COUNT.load(Ordering::Relaxed);

        if terminal.is_interrupted() {
            break;
        }

        terminal.sleep(
            std::time::Duration::from_millis(16)
        );
    }

    terminal.leave();
}
```

In this example:

- `BIND` associates a key with a function.
- `Key::SPACE` selects the space bar.
- `inc_count` increments the counter.
- `process_binds` processes the registered bindings.

## Notifications

TgLib supports desktop notifications on Windows and Unix platforms.

```rust
use tglib::components::Notification;
use tglib::types::NotificationType;

fn main() {
    Notification::new(
        "I am a Title",
        "I am the body",
        NotificationType::Info,
    ).push();
}
```

This creates a notification with a title, body, and notification type.

## Project Structure

The library source code is organised into modules:

```text
src/
├── lib.rs
├── terminal.rs
├── types.rs
├── colors.rs
├── components.rs
└── platform/
    ├── mod.rs
    ├── windows.rs
    └── unix.rs
```

- `lib.rs` exposes the public library API.
- `terminal.rs` contains terminal management.
- `types.rs` defines types used throughout the library.
- `colors.rs` contains colour constants.
- `components.rs` contains UI components.
- `platform/` contains platform-specific implementations.

## Documentation

- [Crates.io](https://crates.io/crates/tglib-rs)
- [API Documentation](https://docs.rs/tglib-rs)
- [GitHub Repository](https://github.com/enesx32/TgLib)

## Status

TgLib is currently in early development, version **0.2.3**.

The API may change as more terminal UI functionality is added.

## Contributing

Contributions, suggestions, and bug reports are welcome.

Feel free to open an issue or submit a pull request on GitHub.

## License

TgLib is licensed under the MIT License.
