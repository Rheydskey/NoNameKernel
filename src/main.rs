#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
#![feature(panic_info_message)]

mod arch;
use crate::arch::x86_64::gdt::gdt_install;
use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};
use core::panic::PanicInfo;

mod lib;
mod print;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut buffer = Writer::default();
    let panic_message = _info.message().unwrap().as_str().unwrap();
    let panic_file = _info.location().unwrap().file();
    let panic_column = _info.location().unwrap().column();
    let panic_line = _info.location().unwrap().line();

    buffer.color_code = ColorCode::new(Color::Red, Color::Black);
    write!(
        buffer,
        "PANIC at {}:{}:{} => {}",
        panic_file, panic_line, panic_column, panic_message
    )
    .unwrap();
    buffer.new_line();
    loop {}
}

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
