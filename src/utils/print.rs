use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};

pub fn _print(buf: &mut Writer, msg: &str) {
    buf.color_code = ColorCode::new(Color::White, Color::Black);
    buf.write_string(msg);
}
pub fn _print_bytes(buf: &mut Writer, msg: u8) {
    buf.write_byte(msg)
}
pub fn _print_color(buf: &mut Writer, msg: &str, color: ColorCode) {
    buf.color_code = color;
    buf.write_string(msg);
}
