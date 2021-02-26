use crate::lib::vga::Writer;
use crate::lib::vga_color::{ColorCode, Color};

pub fn print(buf: &mut Writer, msg: &str) {
    buf.color_code = ColorCode::new(Color::White, Color::Black);
    buf.write_string(msg);
}
pub fn print_bytes(buf: &mut Writer, msg: u8) {
    buf.write_byte(msg)
}
pub fn print_color(buf: &mut Writer, msg: &str, color: ColorCode) {
    buf.color_code = color;
    buf.write_string(msg);
}