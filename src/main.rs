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

mod arch;
mod bootloader;
mod drivers;
mod lib;
mod utils;
mod kernel;

use crate::bootloader::BUFFER_ADDR;
#[cfg(feature = "stivale2")]
use bootloader::stivale2_header::{Stivale2Struct, Stivale2StructInner};

use drivers::vga::{buffer::Writer, render::BUFFER};
use kernel::kmain;


#[cfg(feature = "bootimage")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {BUFFER_ADDR = 0xb8000};
    unsafe { BUFFER.get_or_init(|| Writer::new_with_addr(0xb8000)) };
    kmain();
    loop {}
}

#[cfg(feature = "stivale2")]
pub unsafe fn load(address: usize) -> Stivale2Struct {
    let inner = &*(address as *const Stivale2StructInner);
    Stivale2Struct { inner }
}

#[cfg(feature = "stivale2")]
#[no_mangle]
pub extern "C" fn _start(stivale: usize) -> ! {
    //let stivalestruct  = unsafe {load(stivale)};

    /*let addr = stivalestruct.get_framebuffer();

    if let Some(e) = addr {
        unsafe { BUFFER.get_or_init(|| Writer::new_with_addr(e.framebuffer_addr as u8)) };
    } else {
        loop {}
    }

    //kmain();*/
    loop {}
}