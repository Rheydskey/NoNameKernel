use super::set_entry;
use crate::arch::x86_64::idt::init_idt;
use crate::lib::vga::Writer;
use core::fmt::Write;

const EXCEPTION_MESSAGES: [&str; 31] = [
    "Division by Zero",
    "Debug",
    "Non-Maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bat TSS",
    "Segment not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt",
    "Coprocessor Fault",
    "Alignment Check",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
];
use super::TypeAttr;
use crate::arch::x86_64::idt::TypeAttr::INTGATE;

pub unsafe fn isr_install() {
    for i in 0..48 {
        set_entry(i, __interrupt_vector[i], 0, TypeAttr::INTGATE as u8);
    }

    set_entry(127, __interrupt_vector[48], 0, INTGATE as u8);
    set_entry(128, __interrupt_vector[49], 0, INTGATE as u8 | 0b01100000);

    // Load the IDT to the CPU
    init_idt();

    asm!("sti");
}

#[link(name = "interrupt")]
extern "C" {
    static __interrupt_vector: [usize; 128];
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C, packed)]
pub struct Registers {
    ds: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rsp: u64,
    rdx: u64,
    rbx: u64,
    rax: u64,
    int_no: u64,
    err_code: u64,
    rip: u64,
    cs: u64,
    eflags: u64,
    useresp: u64,
    ss: u64,
}
#[no_mangle]
pub extern "C" fn isr_handler(rps: usize) {
    let mut writer = Writer::default();
    let reg = unsafe { &*(rps as *const Registers) };
    write!(
        writer,
        "[{}] Interrups : {}",
        reg.err_code,
        EXCEPTION_MESSAGES
            .get(reg.int_no as usize)
            .unwrap_or(&"NO MSG")
    );
}
