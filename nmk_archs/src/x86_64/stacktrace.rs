use nmk_utils::asm::read_ebp;

#[derive(Debug)]
#[repr(C)]
pub struct StackTrace {
    rbp: *mut usize,
    rip: usize,
}
impl StackTrace {
    pub unsafe fn get() {
        let ptr_stack: u64 = read_ebp();
        println!("{:#X}", ptr_stack);
        let mut stack: &StackTrace = &*(ptr_stack as *mut usize).cast::<StackTrace>();

        let mut n = 0;
        while let Some(trace) = stack.get_next() {
            println!("{:x}", trace.rip);
            stack = trace;
            if n == 3 {
                break;
            }
            n += 1;
        }
    }

    pub fn get_next(&self) -> Option<&StackTrace> {
        if !self.rbp.is_null() && self.rbp as usize != 0x00 {
            return Some(unsafe { &*self.rbp.cast::<StackTrace>() });
        }

        None
    }
}
