#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(const_raw_ptr_deref)]
#![feature(once_cell)]
#![feature(fn_traits)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_option)]
#![feature(destructuring_assignment)]
#![allow(dead_code)]


use arch::x86_64::{gdt::gdt_init, idt::init_idt};
use drivers::vga::vga_color::{Color, ColorCode};

mod arch;
mod bootloader;
mod drivers;
mod lib;
mod utils;

#[cfg(feature = "stivale")]
static STACK: [u8; 4096] = [0;4096];


pub fn kmain() {
    println_color!(
        ColorCode::new(Color::LightCyan, Color::Black),
        "Welcome on NoName Kernel {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );

    utils::status::Init::new("GDT").wait(gdt_init);
    utils::status::Init::new("IDT").wait(init_idt);
}