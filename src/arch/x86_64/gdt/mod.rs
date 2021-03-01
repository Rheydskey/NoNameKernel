/*
Rewrite of https://raw.githubusercontent.com/hach-que/Kernel/master/gdt.c
By Rheydskey
*/

use lazy_static::lazy_static;

const GDT_SEGMENT: u8 = 0b00010000;
const GDT_PRESENT : u8 =0b10000000;
const GDT_USER : u8 = 0b01100000;
const GDT_EXECUTABLE : u8 = 0b00001000;
const GDT_READWRITE : u8 = 0b00000010;

use core::{intrinsics::transmute, mem::size_of, u32};
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,

    base_low: u16,
    base_middle: u8,

    flags: u8,
    granularity: u8,
    base_high: u8,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct GdtPtr {
    limit: u16,
    base: u64,
}
lazy_static!{
    static ref GDT: [GdtEntry; 5] = [GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    flags: 0,
    granularity: 0,
    base_high: 0,
}; 5];
}

lazy_static!{
    static ref GP: GdtPtr = GdtPtr { limit: 0, base: 0 };
}

pub unsafe fn gdt_set_gate(num: usize, base: u32, limit: u32, flags: u8, gran: u8) {
    let mut gdt = *GDT;
    gdt[num].base_low = (base & 0xffff) as u16;
    gdt[num].base_middle = ((base >> 16) & 0xFF) as u8;
    gdt[num].base_high = ((base >> 24) & 0xff) as u8;

    gdt[num].limit_low =  (limit & 0xffff) as u16;
    gdt[num].granularity = gran;

    gdt[num].granularity |= gran & 0xF0;
    gdt[num].flags = flags;
}

pub unsafe fn init_gdt() {
    let mut gdtptr = *GP;
    gdtptr.limit = (size_of::<GdtEntry>() - 1) as u16;
    gdtptr.base = transmute(&GDT);

    gdt_set_gate(0, 0, 0, 0, 0);
    gdt_set_gate(1, 0, 0, GDT_PRESENT | GDT_SEGMENT | GDT_READWRITE | GDT_EXECUTABLE, 0x10);
    gdt_set_gate(2, 0, 0, GDT_PRESENT | GDT_SEGMENT | GDT_READWRITE, 0);
    /* Install the user mode segments into the GDT */
    gdt_set_gate(3, 0, 0, GDT_PRESENT | GDT_SEGMENT | GDT_READWRITE | GDT_EXECUTABLE | GDT_USER, 0x10);
    gdt_set_gate(4, 0, 0, GDT_PRESENT | GDT_SEGMENT | GDT_READWRITE | GDT_USER, 0);

    _x86_64_lgdt(&gdtptr as *const GdtPtr as u64);
}

#[link(name="x86_64_gdt")]
extern "C" {
    pub fn _x86_64_lgdt(gdtp: u64);
}