/*
Rewrite of https://raw.githubusercontent.com/hach-que/Kernel/master/gdt.c
By Rheydskey
*/

use crate::lib::{
    vga::Writer,
    vga_color::{Color, ColorCode},
};
use crate::utils::print::print_color;
use core::mem::size_of;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u16,
    access: u16,
    granularity: u32,
    base_high: u16,
}

#[derive(Copy, Clone)]
pub struct GdtPtr {
    limit: u16,
    base: u32,
}

pub static mut GDT: [GdtEntry; 5] = [GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0,
    granularity: 0,
    base_high: 0,
}; 5];

pub static mut GP: GdtPtr = GdtPtr { limit: 0, base: 0 };

pub unsafe fn gdt_set_gate(num: usize, base: u32, limit: u32, access: u16, gran: u32) {
    GDT[num].base_low = (base & 0xFFFF) as u16;
    GDT[num].base_middle = ((base >> 16) & 0xFF) as u16;
    GDT[num].base_high = ((base >> 24) & 0xFF) as u16;

    GDT[num].limit_low = (limit & 0xFFFF) as u16;
    GDT[num].granularity = (limit >> 16) & 0x0F;

    GDT[num].granularity |= gran & 0xF0;
    GDT[num].access = access;
}

pub unsafe fn gdt_install(write: &mut Writer) {
    GP.limit = (size_of::<GdtEntry>() * 6 - 1) as u16;
    //GP.base = &*GDT as u64;
    gdt_set_gate(0, 0, 0, 0, 0);
    gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
    /* Install the user mode segments into the GDT */
    gdt_set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
    gdt_set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);
    print_color(
        write,
        "GDT Loaded Succesfully",
        ColorCode::new(Color::Green, Color::Black),
    );
}
