use crate::lib::vga::Writer;

pub fn print(buf: &mut Writer, msg: &str) {
    buf.write_string(msg);
}