static KEYBOARDMAP: [char; 56] = [
    '\0', '\0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\0', '\t', 'q', 'w',
    'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', '\0', 'a', 's', 'd', 'f', 'g', 'h',
    'j', 'k', 'l', ';', '\'', '`', ' ', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/',
    ' ', '*',
];

enum Keyboard {
    Key(char),
    Esc,
    Space,
    Back,
    Shift,
    Ctrl,
    Enter,
    Other,
}

fn translate(scancode: u8) -> Keyboard {
    match scancode {
        0x01 => Keyboard::Esc,
        0x0E => Keyboard::Back,
        0x1C => Keyboard::Enter,
        0x36 | 0x2A => Keyboard::Shift,
        0x39 => Keyboard::Space,
        0x1D => Keyboard::Ctrl,
        _ => {
            if scancode >= u8::try_from(KEYBOARDMAP.len()).unwrap() {
                Keyboard::Other
            } else {
                Keyboard::Key(KEYBOARDMAP[scancode as usize])
            }
        }
    }
}

use crate::{print, println};

pub fn key_handler(scancode: u8) {
    let translate = translate(scancode);

    match translate {
        Keyboard::Key(e) => {
            print!("{}", e);
        }
        Keyboard::Esc => {
            print!("ESC");
        }
        Keyboard::Space => {
            print!(" ");
        }
        Keyboard::Back => {
            print!("Back");
        }
        Keyboard::Shift => {
            print!("SHIFT");
        }
        Keyboard::Ctrl => {
            print!("CTRL");
        }
        Keyboard::Other => {}
        Keyboard::Enter => {
            println!();
        }
    }
}
