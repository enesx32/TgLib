use tglib::Terminal;
use tglib::components::Label;
use tglib::colors::*;
use tglib::types::Text;

fn main() {
    let terminal = Terminal::new();

    // `setup_guard` restores the terminal when it is dropped.
    let _guard = terminal.setup_guard();
    terminal.hide_cursor();

    let mut count = 0;

    while !terminal.is_interrupted() {
        let status = Label {
            pos: (2, 1),
            fore_color: GREEN,
            back_color: BG_BLACK,
            text: Text::from_str(&format!("Count: {}", count)),
        };

        terminal.begin_frame();
        terminal.draw(&status);
        terminal.render();

        count += 1;
        terminal.sleep(std::time::Duration::from_millis(250));
    }
}
