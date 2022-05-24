#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(once_cell)]
#![feature(fn_traits)]
#![feature(abi_x86_interrupt)]
#![feature(const_option)]
#![feature(format_args_nl)]
#![allow(dead_code)]

#[macro_use]
extern crate nmk_drivers;

mod kernel;
mod panic_handler;

use kernel::kmain;
use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader};

const STACK_SIZE: usize = 4096;

static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

static FRAMEBUFFER_TAG: StivaleFramebufferHeaderTag =
    StivaleFramebufferHeaderTag::new().framebuffer_bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALEHDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[STACK_SIZE - 1] as *const u8)
    .tags((&FRAMEBUFFER_TAG as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
pub extern "C" fn _start(_: usize) -> ! {
    kmain();
    loop {}
}
