static KEYBOARDMAP: [char; 56] = [
'\0','\0','1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\0', '\t',
'q', 'w', 'e', 'r','t','y','u', 'i', 'o', 'p', '[', ']', '\n', '\0',
'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'','`', ' ', '\\','z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', ' ', '*',   ];

fn translate(scancode: u8) -> char {

    if scancode >= KEYBOARDMAP.len() as u8{
        '\0'
    } else {
        KEYBOARDMAP[scancode as usize]
    }
}

use crate::print;

pub fn key_handler(scancode: u8) {

    print!("{}", scancode);

    let translate = translate(scancode);

    if translate != '\0' {
        print!("{}", translate);
    }
}