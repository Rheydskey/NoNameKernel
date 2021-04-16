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
#[cfg(feature = "stivale")]
use crate::bootloader::stivale::{StivaleHeader, StivaleStruct};
#[cfg(feature = "stivale")]
use core::lazy::OnceCell;

use drivers::vga::vga_color::{Color, ColorCode};

mod arch;
mod bootloader;
mod drivers;
mod lib;
mod utils;

static STACK: [u8; 4096] = [0; 4096];

#[cfg(feature = "stivale")]


#[cfg(feature = "stivale2")]
use crate::bootloader::stivale2::{Stivale2Struct, Stivale2Header};

pub fn kmain() {
    println_color!(
        ColorCode::new(Color::LightCyan, Color::Black),
        "Welcome on NoName Kernel {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );

    utils::status::Init::new("GDT").wait(gdt_init);
    utils::status::Init::new("IDT").wait(init_idt);

    crate::println!("{:?}", unsafe {&arch::x86_64::memory::pmm::PAGE_DIRECTORY});

    unsafe {
        arch::x86_64::memory::pmm::PAGE_DIRECTORY.entries[1].set(0xFFFFFFFFFFFFFFFF);
        for i in arch::x86_64::memory::pmm::PAGE_DIRECTORY.iter() {
            if i.is_unused() {
                println!("It's free");
            } else {
                println!("{}", i.addr())
            }
        }

    }
}