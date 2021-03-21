#![allow(dead_code)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use crate::arch::x86_64::gdt::init_gdt;
use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};
use crate::utils::status::Init;
use core::fmt::Write;

mod arch;
mod lib;
mod utils;

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
    gdt_status.pending();
    idt_status.pending();
    unsafe {
        init_gdt();
        gdt_status.ok();
    };
    loop {}
}
