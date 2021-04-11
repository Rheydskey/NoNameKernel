#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(const_raw_ptr_deref)]
#![feature(once_cell)]
#![feature(fn_traits)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

use arch::x86_64::{gdt::gdt_init, idt::{init_idt, pit::PIT}};

use drivers::vga::vga_color::{Color, ColorCode};

mod arch;
mod drivers;
mod lib;
mod utils;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println_color!(
        ColorCode::new(Color::LightCyan, Color::Black),
        "Welcome on NoName Kernel {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );

    let mut gdt = utils::status::Init::new("GDT");

    gdt_init();

    gdt.ok();

    let mut idt = utils::status::Init::new("IDT");

    idt.pending();

    init_idt();

    idt.ok();

    loop {
        unsafe {let buf =  crate::drivers::vga::render::BUFFER.get_mut().expect("Error");
        buf.set_position((0, buf._get_position().1))}
        print!("{:?}", unsafe {&PIT });


    }
}
