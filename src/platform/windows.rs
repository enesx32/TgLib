use windows::Win32::System::Console::{
    GetConsoleScreenBufferInfo,
    GetStdHandle,
    SetConsoleCtrlHandler,
    CONSOLE_SCREEN_BUFFER_INFO,
    STD_OUTPUT_HANDLE,
};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState,
    VK_SPACE,
    VK_TAB,
    VK_CAPITAL,
    VK_SHIFT,
    VK_CONTROL,
    VK_LWIN,
    VK_MENU,
    VK_UP,
    VK_DOWN,
    VK_LEFT,
    VK_RIGHT,
    VK_F1,
    VK_F2,
    VK_F3,
    VK_F4,
    VK_F5,
    VK_F6,
    VK_F7,
    VK_F8,
    VK_F9,
    VK_F10,
    VK_F11,
    VK_F12,
};

use crate::types::{ Key, NotificationType };

use std::process::{Command,Stdio };
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

static KEY_STATES: Mutex<[bool; 256]> = Mutex::new([false; 256]);

unsafe extern "system" fn ctrl_c_handler(event: u32) -> windows::core::BOOL {
    if event == 0 {
        INTERRUPTED.store(true, Ordering::SeqCst);
        windows::core::BOOL(1)
    } else {
        windows::core::BOOL(0)
    }
}

pub fn install_ctrl_c_handler() {
    unsafe {
        SetConsoleCtrlHandler(Some(ctrl_c_handler), true)
            .expect("failed to install Ctrl+C handler");
    }
}

pub fn notify(title: &str, body: &str, type_: NotificationType) {
    let icon_name = match type_ {
        NotificationType::Info => "Info",
        NotificationType::Warn => "Warning",
        NotificationType::Error => "Error",
    };

    let script = format!(
        "[void][System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); \
         $n = New-Object System.Windows.Forms.NotifyIcon; \
         $n.Icon = [System.Drawing.SystemIcons]::{}; \
         $n.Visible = $true; \
         $n.ShowBalloonTip(5000, '{}', '{}', [System.Windows.Forms.ToolTipIcon]::{});",
        icon_name, title, body, icon_name
    );

    let _ = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

pub fn setup_keyboard() {}

pub fn restore_keyboard() {}

fn is_pressed_now(vk: i32) -> bool {
    unsafe {
        (GetAsyncKeyState(vk) as u16 & 0x8000) != 0
    }
}

fn check_key(vk: i32, key: Key, states: &mut [bool; 256]) -> Option<Key> {
    if vk < 0 || vk >= 256 {
        return None;
    }

    let index = vk as usize;
    let down = is_pressed_now(vk);
    let was_down = states[index];

    states[index] = down;

    if down && !was_down {
        Some(key)
    } else {
        None
    }
}

pub fn read_key() -> Option<Key> {
    let mut states = KEY_STATES.lock().unwrap();
    let mut result = None;

    let keys = [
        (0x41, Key::A),
        (0x42, Key::B),
        (0x43, Key::C),
        (0x44, Key::D),
        (0x45, Key::E),
        (0x46, Key::F),
        (0x47, Key::G),
        (0x48, Key::H),
        (0x49, Key::I),
        (0x4A, Key::J),
        (0x4B, Key::K),
        (0x4C, Key::L),
        (0x4D, Key::M),
        (0x4E, Key::N),
        (0x4F, Key::O),
        (0x50, Key::P),
        (0x51, Key::Q),
        (0x52, Key::R),
        (0x53, Key::S),
        (0x54, Key::T),
        (0x55, Key::U),
        (0x56, Key::V),
        (0x57, Key::W),
        (0x58, Key::X),
        (0x59, Key::Y),
        (0x5A, Key::Z),

        (0x31, Key::Key1),
        (0x32, Key::Key2),
        (0x33, Key::Key3),
        (0x34, Key::Key4),
        (0x35, Key::Key5),
        (0x36, Key::Key6),
        (0x37, Key::Key7),
        (0x38, Key::Key8),
        (0x39, Key::Key9),
        (0x30, Key::Key0),

        (VK_F1.0 as i32, Key::F1),
        (VK_F2.0 as i32, Key::F2),
        (VK_F3.0 as i32, Key::F3),
        (VK_F4.0 as i32, Key::F4),
        (VK_F5.0 as i32, Key::F5),
        (VK_F6.0 as i32, Key::F6),
        (VK_F7.0 as i32, Key::F7),
        (VK_F8.0 as i32, Key::F8),
        (VK_F9.0 as i32, Key::F9),
        (VK_F10.0 as i32, Key::F10),
        (VK_F11.0 as i32, Key::F11),
        (VK_F12.0 as i32, Key::F12),

        (VK_TAB.0 as i32, Key::TAB),
        (VK_CAPITAL.0 as i32, Key::CAPS),
        (VK_SHIFT.0 as i32, Key::SHIFT),
        (VK_CONTROL.0 as i32, Key::CTRL),
        (VK_LWIN.0 as i32, Key::WIN),
        (VK_MENU.0 as i32, Key::ALT),
        (VK_SPACE.0 as i32, Key::SPACE),

        (VK_UP.0 as i32, Key::UP),
        (VK_DOWN.0 as i32, Key::DOWN),
        (VK_LEFT.0 as i32, Key::LEFT),
        (VK_RIGHT.0 as i32, Key::RIGHT),
    ];

    for (vk, key) in keys {
        if let Some(pressed) = check_key(vk, key, &mut states) {
            if result.is_none() {
                result = Some(pressed);
            }
        }
    }

    result
}

pub fn clear_screen() {
    Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .expect("failed to execute process");
}

pub fn get_terminal_height() -> u16 {
    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe {
        std::mem::zeroed()
    };

    let handle = match unsafe {
        GetStdHandle(STD_OUTPUT_HANDLE)
    } {
        Ok(handle) => handle,
        Err(_) => return 24,
    };

    match unsafe {
        GetConsoleScreenBufferInfo(handle, &mut csbi)
    } {
        Ok(_) => {
            (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16
        }

        Err(_) => 24,
    }
}



pub fn get_terminal_width() -> u16 {
    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe {
        std::mem::zeroed()
    };

    let handle = match unsafe {
        GetStdHandle(STD_OUTPUT_HANDLE)
    } {
        Ok(handle) => handle,
        Err(_) => return 80,
    };

    match unsafe {
        GetConsoleScreenBufferInfo(handle, &mut csbi)
    } {
        Ok(_) => {
            (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16
        }

        Err(_) => 80,
    }
}