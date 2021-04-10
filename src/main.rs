#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_mut_refs)]
#![feature(const_raw_ptr_deref)]
#![feature(const_maybe_uninit_assume_init)]
#![feature(once_cell)]

mod arch;
mod drivers;
mod lib;
mod utils;

use crate::drivers::vga::buffer::Writer;
use crate::drivers::vga::vga_color::{Color, ColorCode};
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut buffer = Writer::default();
    buffer.color_code = ColorCode::new(Color::LightCyan, Color::Black);
    write!(
        buffer,
        "Welcome on NoName Kernel {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    )
    .unwrap();

    print!("Tetet");

    loop {}
}
