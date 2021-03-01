//#[allow(arithmetic_overflow)]

use core::mem::transmute;
use core::{mem::size_of, u16};
use lazy_static::lazy_static;

const INTGATE: i32 = 0x8e;
const TRAPGATE: i32 = 0x8e;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtPrt {
    limit: u16,
    base: u64,
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
impl Default for IdtEntry {
    fn default() -> Self {
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
lazy_static! {
    static ref IDTENTRIES: [IdtEntry; 256] = [IdtEntry::default(); 256];
    static ref IDTPTR: IdtPrt = IdtPrt { limit: 0, base: 0 };
};

pub unsafe fn set_entry(num: usize, select: u16, offset: usize, type_attr: u8) {
    let mut idtd = *IDTENTRIES;

    idtd[num].offset_low = offset as u16;
    idtd[num].selector = select;
    idtd[num].type_attr = type_attr;
    idtd[num].offset_mid = (offset >> 16 & 0xffff) as u16;
    idtd[num].offset_high = ((offset >> 32) as u32 & 0xffff0000_u32) as u32;
    idtd[num].zero = 0;
}

pub unsafe fn init_idt() {
    let mut idt_ptr = *IDTPTR;
    idt_ptr.base = transmute(&*IDTENTRIES);
    idt_ptr.limit = (size_of::<IdtEntry>() * 256) as u16;

    set_entry(0, 0, 0, 0);

    _x86_64_lidt(transmute(&IDTENTRIES));
}

#[link(name = "x86_64_idt")]
extern "C" {
    pub fn _x86_64_lidt(idtp: *const IdtPrt);
}
