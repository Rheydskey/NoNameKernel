static mut BUFFER: Option<buffer::Writer> = None;

pub mod buffer;
pub mod new_vga;
pub mod render;
pub mod vga_color;
