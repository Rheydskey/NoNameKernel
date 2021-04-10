#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_mut_refs)]
#![feature(const_raw_ptr_deref)]
#![feature(const_maybe_uninit_assume_init)]
#![feature(once_cell)]

use drivers::vga::vga_color::{Color, ColorCode};

mod arch;
mod drivers;
mod lib;
mod utils;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println_color!(ColorCode::new(Color::LightCyan,Color::Black), "Welcome on NoName Kernel {}-{}", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));

    print!("New features soon :)");

    loop {}
}
