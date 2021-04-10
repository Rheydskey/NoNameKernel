use core::{mem::size_of, usize};

use bitflags::bitflags;

const IDT_ENTRIES : usize = 256;
static mut IDT: [IDTEntry; IDT_ENTRIES] = [IDTEntry::null(); IDT_ENTRIES];

#[repr(C, packed)]
pub struct IDTPtr {
    size: u16,
    base: u64,
}

impl IDTPtr {
    /// Create a new IDT descriptor.
    #[inline]
    const fn new(size: u16, base: u64) -> Self {
        Self { size, base }
    }
}

bitflags! {
    pub struct IDTFlags: u8 {
        const PRESENT = 1 << 7;
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        const SS = 1 << 4;
        const INTERRUPT = 0xE;
        const TRAP = 0xF;
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: usize,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: usize,
    pub stack_segment: u64,
}

type HandlerInterrupt = unsafe extern "x86-interrupt" fn(InterruptStackFrame);

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct IDTEntry {
    offset_l16: u16,
    select: u16,
    ist: u8,
    type_attr: u8,
    offset_m16: u16,
    offset_h32: u32,
    zero: u32
}


impl IDTEntry {
    pub const fn null() -> Self {
        Self {
            offset_l16: 0,
            select: 0,
            ist: 0,
            type_attr: 0,
            offset_m16: 0,
            offset_h32: 0,
            zero: 0,
        }
    }
    fn set_flags(&mut self, flags: IDTFlags) {
        self.type_attr = flags.bits;
    }

    fn set_offset(&mut self, selector: u16, base: usize) {
        self.select = selector;
        self.offset_l16 = base as u16;
        self.offset_m16 = (base >> 16) as u16;
        self.offset_h32 = (base >> 32) as u32;
    }

    pub fn set_function(&mut self, handler: HandlerInterrupt) {
        self.set_flags(IDTFlags::PRESENT | IDTFlags::RING_0 | IDTFlags::INTERRUPT);
        self.set_offset(8,handler as usize)
    }
}

pub unsafe fn load_idt(idtptr: *const IDTPtr) {
    asm!("lidt [{}]", in(reg) idtptr, options(nostack));
}
pub fn init_idt() {
    unsafe {
        IDT[0].set_function(super::exceptions::divide_by_zero);
        IDT[1].set_function(super::exceptions::debug);
        IDT[2].set_function(super::exceptions::non_maskable);
        IDT[3].set_function(super::exceptions::breakpoint);
        IDT[4].set_function(super::exceptions::overflow);
        IDT[5].set_function(super::exceptions::bound_range);
        IDT[6].set_function(super::exceptions::invalid_opcode);
        IDT[7].set_function(super::exceptions::device_not_available);
        IDT[8].set_function(super::exceptions::double_fault);
        IDT[10].set_function(super::exceptions::invalid_tss);
        IDT[11].set_function(super::exceptions::segment_not_present);
        IDT[12].set_function(super::exceptions::stack_segment);
        IDT[13].set_function(super::exceptions::protection);
        IDT[14].set_flags(IDTFlags::PRESENT | IDTFlags::RING_0 | IDTFlags::INTERRUPT);
        IDT[14].set_offset(8, super::exceptions::page_fault as usize);
        IDT[16].set_function(super::exceptions::fpu_fault);
        IDT[17].set_function(super::exceptions::alignment_check);
        IDT[18].set_function(super::exceptions::machine_check);
        IDT[19].set_function(super::exceptions::simd);
        IDT[20].set_function(super::exceptions::virtualization);
        IDT[30].set_function(super::exceptions::security);

        let idt_descriptor = IDTPtr::new(
            ((IDT.len() * size_of::<IDTEntry>()) - 1) as u16,
            (&IDT as *const _) as u64,
        );

        load_idt(&idt_descriptor as *const _);
    }
}

#[inline(always)]
pub unsafe fn disable_interrupts() {
    asm!("cli");
}

#[inline(always)]
pub unsafe fn _enable_interrupts() {
    asm!("sti");
}

#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt", options(nomem, nostack));
}