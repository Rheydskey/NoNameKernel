#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]

mod arch;
use crate::arch::x86_64::gdt::gdt_install;
use crate::lib::vga::Writer;
use crate::print::print;
use core::panic::PanicInfo;

mod lib;
mod print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let hello = b"Welcome on NoName Kernel";
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let mut buffer = Writer::default();
    unsafe { gdt_install(&mut buffer) };
    loop {}
}
