#[macro_export]
macro_rules! interrupt {
    ($fnname:tt, unsafe $code:block) => {
        pub extern "x86-interrupt" fn $fnname(_interrupt: $crate::arch::x86_64::idt::InterruptStackFrame) {
            {$code}
        }
    };

    ($fnname:tt, $code:block) => {
        pub extern "x86-interrupt" fn $fnname(_interrupt: $crate::arch::x86_64::idt::InterruptStackFrame) {
            {$code}
        }
    };
}

use super::InterruptStackFrame;

#[macro_export]
macro_rules! expection_interrupt {
    ($arg:ident,  $interruptname:expr) => {
        crate::interrupt!($arg, {panic!($interruptname)});
    };
}

expection_interrupt!(divide_by_zero, "divide_by_zero");
expection_interrupt!(debug, "debug");
expection_interrupt!(non_maskable, "non_maskable");
expection_interrupt!(breakpoint, "breakpoint");
expection_interrupt!(overflow, "overflow");
expection_interrupt!(bound_range, "bound_range");
expection_interrupt!(invalid_opcode, "invalid_opcode");
expection_interrupt!(device_not_available, "device_not_available");
expection_interrupt!(double_fault, "double_fault");

expection_interrupt!(invalid_tss, "invalid_tss");
expection_interrupt!(segment_not_present, "segment_not_present");
expection_interrupt!(stack_segment, "stack_segment");
expection_interrupt!(protection, "protection");

expection_interrupt!(fpu_fault, "fpu_fault");
expection_interrupt!(alignment_check, "alignment_check");
expection_interrupt!(machine_check, "machine_check");
expection_interrupt!(simd, "simd");
expection_interrupt!(virtualization, "virtualization");
expection_interrupt!(security, "security");

use crate::println;

pub extern "x86-interrupt" fn page_fault(interrupt: InterruptStackFrame) {
    println!("IntPointer : {:#X}, SegPointer: {:#X},StackSeg:{} CodeSeg:{}, Flags: {}", interrupt.instruction_pointer,interrupt.stack_pointer, interrupt.stack_segment,interrupt.code_segment, interrupt.cpu_flags);
    panic!("PAGE FAULT");
}