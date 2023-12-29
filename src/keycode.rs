//! List of keycodes.

use std::str::FromStr;

use kle_serial::f32::Key;

/// A list of supported keys that we can query from the OS. Outside of mod.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[allow(missing_docs)]
pub enum Keycode {
    None,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Command,
    LOption,
    ROption,
    LMeta,
    RMeta,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    CapsLock,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,

    // The following keys have not been tested on MacOS!
    // Numpad keys which have not been implemented: NumpadSeparator NumLock
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadSubtract,
    NumpadAdd,
    NumpadDivide,
    NumpadMultiply,

    // The following keys names represent the position of the key in a US keyboard,
    // not the sign value. In a different keyboards and OS, the position can vary.
    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    BackSlash,
    Semicolon,
    Apostrophe,
    Comma,
    Dot,
    Slash,
}

impl FromStr for Keycode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Key0" | "0" => Ok(Self::Key0),
            "Key1" | "1" => Ok(Self::Key1),
            "Key2" | "2" => Ok(Self::Key2),
            "Key3" | "3" => Ok(Self::Key3),
            "Key4" | "4" => Ok(Self::Key4),
            "Key5" | "5" => Ok(Self::Key5),
            "Key6" | "6" => Ok(Self::Key6),
            "Key7" | "7" => Ok(Self::Key7),
            "Key8" | "8" => Ok(Self::Key8),
            "Key9" | "9" => Ok(Self::Key9),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "H" => Ok(Self::H),
            "I" => Ok(Self::I),
            "J" => Ok(Self::J),
            "K" => Ok(Self::K),
            "L" => Ok(Self::L),
            "M" => Ok(Self::M),
            "N" => Ok(Self::N),
            "O" => Ok(Self::O),
            "P" => Ok(Self::P),
            "Q" => Ok(Self::Q),
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            "T" => Ok(Self::T),
            "U" => Ok(Self::U),
            "V" => Ok(Self::V),
            "W" => Ok(Self::W),
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            "F1" => Ok(Self::F1),
            "F2" => Ok(Self::F2),
            "F3" => Ok(Self::F3),
            "F4" => Ok(Self::F4),
            "F5" => Ok(Self::F5),
            "F6" => Ok(Self::F6),
            "F7" => Ok(Self::F7),
            "F8" => Ok(Self::F8),
            "F9" => Ok(Self::F9),
            "F10" => Ok(Self::F10),
            "F11" => Ok(Self::F11),
            "F12" => Ok(Self::F12),
            "Escape" | "Esc" => Ok(Self::Escape),
            "Space" => Ok(Self::Space),
            "LControl" | "Ctrl" => Ok(Self::LControl),
            "RControl" => Ok(Self::RControl),
            "LShift" | "Shift" => Ok(Self::LShift),
            "RShift" => Ok(Self::RShift),
            "LAlt" | "Alt" => Ok(Self::LAlt),
            "RAlt" => Ok(Self::RAlt),
            "Command" => Ok(Self::Command),
            "LOption" => Ok(Self::LOption),
            "ROption" => Ok(Self::ROption),
            "LMeta" | "Super" => Ok(Self::LMeta),
            "RMeta" => Ok(Self::RMeta),
            "Enter" | "Return" => Ok(Self::Enter),
            "Up" => Ok(Self::Up),
            "Down" => Ok(Self::Down),
            "Left" => Ok(Self::Left),
            "Right" => Ok(Self::Right),
            "Backspace" | "Back Space" => Ok(Self::Backspace),
            "CapsLock" => Ok(Self::CapsLock),
            "Tab" => Ok(Self::Tab),
            "Home" => Ok(Self::Home),
            "End" => Ok(Self::End),
            "PageUp" => Ok(Self::PageUp),
            "PageDown" => Ok(Self::PageDown),
            "Insert" => Ok(Self::Insert),
            "Delete" => Ok(Self::Delete),
            "Numpad0" => Ok(Self::Numpad0),
            "Numpad1" => Ok(Self::Numpad1),
            "Numpad2" => Ok(Self::Numpad2),
            "Numpad3" => Ok(Self::Numpad3),
            "Numpad4" => Ok(Self::Numpad4),
            "Numpad5" => Ok(Self::Numpad5),
            "Numpad6" => Ok(Self::Numpad6),
            "Numpad7" => Ok(Self::Numpad7),
            "Numpad8" => Ok(Self::Numpad8),
            "Numpad9" => Ok(Self::Numpad9),
            "NumpadSubtract" => Ok(Self::NumpadSubtract),
            "NumpadAdd" | "+" => Ok(Self::NumpadAdd),
            "NumpadDivide" => Ok(Self::NumpadDivide),
            "NumpadMultiply" | "*" => Ok(Self::NumpadMultiply),
            "Grave" | "`" => Ok(Self::Grave),
            "Minus" | "-" => Ok(Self::Minus),
            "Equal" | "=" => Ok(Self::Equal),
            "LeftBracket" | "{" => Ok(Self::LeftBracket),
            "RightBracket" | "}" => Ok(Self::RightBracket),
            "BackSlash" | "\\" => Ok(Self::BackSlash),
            "Semicolon" | ";" => Ok(Self::Semicolon),
            "Apostrophe" | "'" => Ok(Self::Apostrophe),
            "Comma" | "," => Ok(Self::Comma),
            "Dot" | "." => Ok(Self::Dot),
            "Slash" | "/" => Ok(Self::Slash),
            "" | " " => Ok(Self::None),
            _ => Err(String::from("failed to parse keycode")),
        }
    }
}

impl ToString for Keycode {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

impl From<Key> for Keycode {
    fn from(value: Key) -> Self {
        let keycodes = value
            .legends
            .iter()
            .filter_map(|v| v.as_ref().and_then(|v| Keycode::from_str(&v.text).ok()))
            .collect::<Vec<_>>();
        *keycodes.first().unwrap()
    }
}
