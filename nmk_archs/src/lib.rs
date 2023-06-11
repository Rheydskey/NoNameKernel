#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(format_args_nl)]

#[macro_use]
extern crate nmk_drivers;

pub mod x86_64;

pub mod tests;
