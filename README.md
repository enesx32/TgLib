# TgLib

A lightweight terminal UI library for Rust.

TgLib is designed to make building terminal-based applications easier by providing simple abstractions for terminal management, text, colours, and components.

> **Current version: 0.2.3**

## Features

* Terminal size detection
* Cross-platform terminal support
* Screen clearing
* Coloured text
* Background colours
* Text components
* Simple terminal UI structure
* Windows and Unix support
* Added Keybind support

## Requirements

* Rust
* Cargo
* Git

## Installation & Running

### Windows — PowerShell

```powershell
git clone https://github.com/enes73546/TgLib.git; cd TgLib; cargo add windows; cargo run
```

### Linux / macOS — Bash

```bash
git clone https://github.com/enes73546/TgLib.git && cd TgLib && cargo add libc && cargo run
```

## Platform Dependencies

TgLib uses platform-specific libraries to interact with the terminal.

### Windows

Windows support uses the `windows` crate.

If you are building TgLib on Windows, install it with:

```powershell
cargo add windows
```

### Unix / Linux / macOS

Unix-based systems use the `libc` crate.

Install it with:

```bash
cargo add libc
```

`libc` is not part of the Rust standard library, so it must be included as a Cargo dependency.

## Basic Example - Found in main.rs

```rust
use t_glib::Terminal;
use t_glib::components::Label;
use t_glib::colors::*;
use t_glib::types::Text;
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
                text: Text::from_str(
                    "Exiting Program"
                ),
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

## Project Structure

```text
src/
├── lib.rs
├── main.rs
├── terminal.rs
├── types.rs
├── colors.rs
├── components.rs
└── platform/
    ├── mod.rs
    ├── windows.rs
    └── unix.rs
```

## Colours

TgLib provides ANSI colour constants such as:

```rust
use t_glib::colors::*;

GREEN
RED
BLUE
WHITE
BG_BLACK
BG_RED
RESET
```

More colours and styles are available in the `colors` module.

## Terminal Coordinates

TgLib uses `(x, y)` for positions.

For example:

```rust
pos: (40, 3)
```

means:

```py
x = 40
y = 3
```

The coordinate system is used when positioning components inside the terminal.

## Pixels and Cells

In the TgLib (Terminal Graphics Library) API a Pixel is a single space in the terminal which can have a color 
however a cell can have a character inside it with its own color

> * You can see this in further depth in `TgLib v0.2.1`

```rs
use t_glib::components::{ Label, Pixel, PixelSet };
use t_glib::colors::*;

fn main() {
    let pixel_set = PixelSet { 
            pixels: vec![
                Pixel { pos: (40, 5), color: YELLOW }, Pixel { pos: (41, 5), color: YELLOW },
                Pixel { pos: (42, 5), color: YELLOW }, Pixel { pos: (43, 5), color: YELLOW },
                Pixel { pos: (44, 5), color: YELLOW }, Pixel { pos: (45, 5), color: YELLOW },
                Pixel { pos: (46, 5), color: YELLOW }, Pixel { pos: (47, 5), color: YELLOW },
                Pixel { pos: (48, 5), color: YELLOW }, Pixel { pos: (49, 5), color: YELLOW },

                Pixel { pos: (40, 6), color: YELLOW }, Pixel { pos: (41, 6), color: YELLOW },
                Pixel { pos: (42, 6), color: BLACK }, Pixel { pos: (43, 6), color: BLACK },
                Pixel { pos: (44, 6), color: YELLOW }, Pixel { pos: (45, 6), color: YELLOW },
                Pixel { pos: (46, 6), color: BLACK }, Pixel { pos: (47, 6), color: BLACK },
                Pixel { pos: (48, 6), color: YELLOW }, Pixel { pos: (49, 6), color: YELLOW },

                Pixel { pos: (40, 7), color: YELLOW }, Pixel { pos: (41, 7), color: YELLOW },
                Pixel { pos: (42, 7), color: YELLOW }, Pixel { pos: (43, 7), color: YELLOW },
                Pixel { pos: (44, 7), color: YELLOW }, Pixel { pos: (45, 7), color: YELLOW },
                Pixel { pos: (46, 7), color: YELLOW }, Pixel { pos: (47, 7), color: YELLOW },
                Pixel { pos: (48, 7), color: YELLOW }, Pixel { pos: (49, 7), color: YELLOW },

                Pixel { pos: (40, 8), color: YELLOW }, Pixel { pos: (41, 8), color: YELLOW },
                Pixel { pos: (42, 8), color: BLACK }, Pixel { pos: (43, 8), color: BLACK },
                Pixel { pos: (44, 8), color: BLACK }, Pixel { pos: (45, 8), color: BLACK },
                Pixel { pos: (46, 8), color: BLACK }, Pixel { pos: (47, 8), color: BLACK },
                Pixel { pos: (48, 8), color: YELLOW }, Pixel { pos: (49, 8), color: YELLOW },

                Pixel { pos: (40, 9), color: YELLOW }, Pixel { pos: (41, 9), color: YELLOW },
                Pixel { pos: (42, 9), color: YELLOW }, Pixel { pos: (43, 9), color: YELLOW },
                Pixel { pos: (44, 9), color: YELLOW }, Pixel { pos: (45, 9), color: YELLOW },
                Pixel { pos: (46, 9), color: YELLOW }, Pixel { pos: (47, 9), color: YELLOW },
                Pixel { pos: (48, 9), color: YELLOW }, Pixel { pos: (49, 9), color: YELLOW },
            ],
        };

    terminal.show_pixelset(&pixel_set);
}
```

Although this may look confusing it prints a smiley face into the terminal when ran

Cells are a little bit different though as they do not contain a cellset component
to init a cell you must run `let my_cell = Cell { pos: (1,1), color: BG_RED, char_: '#', char_color: BLUE }`
then run `show_cell(my_cell)` after

## Key binding

You can bind keys with

```rs
use t_glib::types::Key;
use t_glib::components::BIND;

static COUNT: AtomicUsize = AtomicUsize::new(0); 

fn inc_count() {
    COUNT.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    let counter_bind = BIND { // Create the variable
        func: inc_count,      // Init the function it runs   
        key: Key::SPACE,      // Init what key runs the function
    };

    terminal.bind(counter_bind);  // Bind the key to the function

    terminal.process_binds();     // Proccess all initialised binds
    let count: usize = COUNT.load(Ordering::Relaxed); // Update the COUNT variable in this example
}
```

## Notifications

TgLib allows you to trigger native desktop notifications across Windows and Unix platforms without extra dependencies.

```rust
use t_glib::components::Notification;
use t_glib::types::NotificationType;

fn main() {
    Notification::new(
        "I am a Title",
        "I am the body",
        NotificationType::Info,
    ).push();
}
```

## Status

TgLib is currently in **early development (0.2.3)**.

The API may change significantly as more terminal UI functionality is added.

## Contributing

Contributions, suggestions, and bug reports are welcome.

If you find a bug or have an idea for a feature, feel free to open an issue or pull request.

## License

License information will be added in a future release.
Probably not tho XD
