use core::fmt::{self};

use super::vga_color::{Color, ColorCode};
use super::BUFFER;
use core::fmt::Write;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe {
        if let Some(e) = &mut BUFFER {
            e.write_fmt(args).expect("Error");
        }
    }
}
