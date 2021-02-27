/*
Rewrite of https://raw.githubusercontent.com/hach-que/Kernel/master/gdt.c
By Rheydskey
*/

use lazy_static::lazy_static;

use crate::lib::{
    vga::Writer,
    vga_color::{Color, ColorCode},
};
use crate::utils::print::print_color;
use core::{intrinsics::transmute, mem::size_of};
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,

    base_low: u16,
    base_middle: u8,

    access: u8,
    granularity: u8,
    base_high: u8,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GdtPtr {
    limit: u16,
    base: usize,
}
lazy_static!{
    static ref GDT: [GdtEntry; 5] = [GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0,
    granularity: 0,
    base_high: 0,
}; 5];
}

lazy_static!{
    static ref GP: GdtPtr = GdtPtr { limit: 0, base: 0 };
}

pub unsafe fn gdt_set_gate(num: usize, base: u32, limit: u32, access: u8, gran: u32) {
    let mut gdt = *GDT;
    gdt[num].base_low = (base & 0xFFFF) as u16;
    gdt[num].base_middle = ((base >> 16) & 0xFF) as u8;
    gdt[num].base_high = ((base >> 24) & 0xFF) as u8;

    gdt[num].limit_low = (limit & 0xFFFF) as u16;
    gdt[num].granularity = ((limit >> 16) & 0x0F) as u8;

    gdt[num].granularity |= (gran & 0xF0) as u8;
    gdt[num].access = access;
}

pub unsafe fn gdt_install(write: &mut Writer) {
    let mut gdtptr = *GP;
    gdtptr.limit = (size_of::<GdtEntry>() * 6 - 1) as u16;
    gdtptr.base = transmute(&GDT);

    gdt_set_gate(0, 0, 0, 0, 0);
    gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
    /* Install the user mode segments into the GDT */
    gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
    gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);

    _x86_64_lgdt(&gdtptr as *const GdtPtr);

    print_color(
        write,
        "[ OK ] GDT Succesfully Loaded",
        ColorCode::new(Color::Green, Color::Black),
    );
}

#[link(name="x86_64_gdt")]
extern "C" {
    pub fn _x86_64_lgdt(gdtp: *const GdtPtr);
}