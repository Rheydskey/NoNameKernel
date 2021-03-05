use lazy_static::lazy_static;
pub mod gdt;

use crate::arch::x86_64::gdt::gdt::{GDT, GDTPointer, GDTSelector, GDTEntry, GDTFlags, GDTGranularity};
use crate::lib::vga::Writer;
use core::fmt::Write;
use core::mem::transmute;

lazy_static!{
    static ref GDTPTR: GDTPointer = GDTPointer::default();
    static ref GDTREF: GDT = GDT::new();
}


pub unsafe fn init_gdt() {
    let mut gdt = *GDTREF;
    let mut gdtptr = *GDTPTR;

    gdt.zero();
    gdt.set(1, GDTEntry::new(GDTFlags::CS as u8 | GDTFlags::WRITABLE as u8, GDTGranularity::LongModeGranularity as u8));
    gdt.set(2, GDTEntry::new(GDTFlags::DS as u8 | GDTFlags::WRITABLE as u8, 0));
    gdt.set(3, GDTEntry::new(GDTFlags::CS as u8 | GDTFlags::USER as u8 | GDTFlags::WRITABLE as u8, GDTGranularity::LongModeGranularity as u8));
    gdt.set(4, GDTEntry::new(GDTFlags::CS as u8 | GDTFlags::USER as u8 | GDTFlags::WRITABLE as u8, 0));

    gdtptr.register(gdt);

    _x86_64_lgdt(&*GDTPTR as *const _ as usize, GDTSelector::KernelCode as u16, GDTSelector::KernelData as u16);
}

#[link(name = "x86_64_gdt")]
extern "C" {
    pub fn _x86_64_lgdt(gdtptr: usize, segment1: u16, segement2: u16);
}
