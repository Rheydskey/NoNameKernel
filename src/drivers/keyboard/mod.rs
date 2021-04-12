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
        1 => Keyboard::Esc,
        14 => Keyboard::Back,
        28 => Keyboard::Enter,
        54 | 42 => Keyboard::Shift,
        57 => Keyboard::Space,
        157 | 224  => Keyboard::Ctrl,
        _ => {
            if scancode >= KEYBOARDMAP.len() as u8 {
                Keyboard::Other
            } else {
                Keyboard::Key(KEYBOARDMAP[scancode as usize])
            }
        }
    }
}

use crate::print;

pub fn key_handler(scancode: u8) {
    let translate = translate(scancode);

    match translate {
        Keyboard::Key(e) => {
            print!("{}", e)
        }
        Keyboard::Esc => {print!("ESC")}
        Keyboard::Space => {print!(" ")}
        Keyboard::Back => {print!("BACK")}
        Keyboard::Shift => {print!("SHIFT")}
        Keyboard::Ctrl => {print!("CTRL")}
        Keyboard::Other => {}
        Keyboard::Enter => {print!("\n")}
    }
}
