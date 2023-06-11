#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
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
use limine::LimineBootInfoRequest;

#[used]
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kmain();
}
