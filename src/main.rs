#![no_std]
#![no_main]
#![feature(asm)]
#![feature(panic_info_message)]


use crate::arch::x86_64::gdt::gdt_install;
use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};
use core::fmt::Write;

mod arch;
mod utils;
mod lib;

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
    buffer.new_line();
    unsafe { gdt_install(&mut buffer) };
    loop {}
}
