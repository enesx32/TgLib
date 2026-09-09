use t_glib::Terminal;
use t_glib::components::Notification;
use t_glib::types::NotificationType;
use std::time::Duration;

fn main() {
    let terminal = Terminal::new();

    terminal.setup();

    let my_notification = Notification::new("Hello", "Welcome to TgLib | v0.2.5", NotificationType::Info,);

    my_notification.push();

    loop {
        terminal.hide_cursor();

        if terminal.is_interrupted() {
            let interrupt_notification = Notification::new("Good Bye", "Exiting Program", NotificationType::Error,);
            interrupt_notification.push();

            terminal.sleep(Duration::from_secs(2));
            terminal.leave();

            break;
        }

        terminal.sleep(Duration::from_millis(16));
    }
}