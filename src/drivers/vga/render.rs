use crate::drivers::vga::buffer::Writer;
use core::{
    fmt::{self, Write},
    lazy::OnceCell,
};

use super::vga_color::ColorCode;

pub static mut BUFFER: core::lazy::OnceCell<Writer> = OnceCell::new();

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe {
        BUFFER.get_or_init(Writer::default);

        if let Some(e) = BUFFER.get_mut() {
            e.write_fmt(args).expect("Error");
        }
    }
}

#[doc(hidden)]
pub fn _print_color(args: fmt::Arguments, color: ColorCode) {
    unsafe {
        BUFFER.get_or_init(Writer::default);

        if let Some(e) = BUFFER.get_mut() {
            e.color_code = color;
            e.write_fmt(args).expect("Error");
            e.color_code = ColorCode::new(
                super::vga_color::Color::White,
                super::vga_color::Color::Black,
            );
        } else {
            panic!("Can't get Buffer");
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga::render::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_color {
    ($color:expr, $($arg:tt)*) => ($crate::drivers::vga::render::_print_color(format_args!($($arg)*), $color));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_color {
    () => ($crate::println!());
    ($color:expr, $($arg:tt)*) => ($crate::print_color!($color, "{}\n", format_args!($($arg)*)));
}
