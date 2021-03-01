#![no_std]
#![no_main]
#![feature(asm)]
#![feature(panic_info_message)]


use crate::arch::x86_64::gdt::init_gdt;
use crate::arch::x86_64::idt::init_idt;
use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};
use core::fmt::Write;
use crate::utils::status::Init;

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
    let mut gdt_status = Init::new(buffer.get_position(), "GDT");
    buffer.new_line();
    let mut idt_status = Init::new(buffer.get_position(), "IDT");
    buffer.new_line();
    let mut error_test = Init::new(buffer.get_position(), "Fake Error");
    gdt_status.pending();
    idt_status.pending();
    unsafe {init_gdt(); init_idt();};
    /*unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };*/
    gdt_status.ok();
    idt_status.ok();
    error_test.error();
    loop {}
}
