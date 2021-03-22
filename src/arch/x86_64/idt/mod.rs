pub mod interrupts;

use core::mem::transmute;
use core::{mem::size_of, u16};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtPtr {
    limit: u16,
    base: u64,
}

enum TypeAttr {
    IntGate = 0x8e,
    TrapGate = 0xef,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}
impl IdtEntry {
    const fn default() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }
}

static IDTENTRIES: &[IdtEntry; 256] = &[IdtEntry::default(); 256];
static IDTPTR: &IdtPtr = &IdtPtr { limit: 0, base: 0 };

pub unsafe fn set_entry(num: usize, offset: usize, ist: u8, attr: u8) {
    let mut idtd = *IDTENTRIES;
    let entry = IdtEntry {
        offset_low: (offset & 0xFFFF) as u16,
        selector: 0x08,
        ist,
        type_attr: attr,
        offset_mid: ((offset >> 16) & 0xFFFF) as u16,
        offset_high: (offset >> 32 & 0xFFFFFFFF) as u32,
        zero: 0,
    };

    idtd[num] = entry;
}

pub unsafe fn init_idt() {
    let mut idt_ptr = *IDTPTR;

    idt_ptr.base = transmute(&IDTENTRIES[0]);
    idt_ptr.limit = size_of::<IdtEntry>() as u16 * 256 - 1;

    _x86_64_lidt(transmute(&IDTPTR));
}

#[link(name = "x86_64_idt")]
extern "C" {
    pub fn _x86_64_lidt(idtp: *const IdtPtr);
}
