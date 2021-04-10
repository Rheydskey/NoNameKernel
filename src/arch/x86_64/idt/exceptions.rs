use super::disable_interrupts;
use super::InterruptStackFrame;
use crate::println;
#[macro_export]
macro_rules! interrupt {
    ($arg:tt) => {
        pub extern "x86-interrupt" fn $arg(interrupt: InterruptStackFrame) {
            println!("Error : {:?}", interrupt);
            unsafe { disable_interrupts() };
        }
    };
}

interrupt!(divide_by_zero);
interrupt!(debug);
interrupt!(non_maskable);
interrupt!(breakpoint);
interrupt!(overflow);
interrupt!(bound_range);
interrupt!(invalid_opcode);
interrupt!(device_not_available);
interrupt!(double_fault);

interrupt!(invalid_tss);
interrupt!(segment_not_present);
interrupt!(stack_segment);
interrupt!(protection);

interrupt!(page_fault);

interrupt!(fpu_fault);
interrupt!(alignment_check);
interrupt!(machine_check);
interrupt!(simd);
interrupt!(virtualization);
interrupt!(security);
