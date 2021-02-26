#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
#![feature(panic_info_message)]

mod arch;
use crate::arch::x86_64::gdt::gdt_install;
use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};
use core::panic::PanicInfo;
mod utils;
mod lib;

use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut buffer = Writer::default();
    let panic_message = match _info.message() {
        Some(arg) => arg.as_str().unwrap_or("No Message Error"),
        None => "No message Error",
    };
    let location = match _info.location() {
        Some(e) => e,
        None => {
            buffer.color_code = ColorCode::new(Color::Red, Color::Black);
            write!(buffer, "No Panic Location => {}", panic_message).unwrap();
            buffer.new_line();
            loop {};
        }
    };
    let panic_file = location.file();
    let panic_column = location.column();
    let panic_line = location.line();
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
