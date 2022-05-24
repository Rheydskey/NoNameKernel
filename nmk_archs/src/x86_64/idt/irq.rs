use super::{pit::PIT, EOI_pic1};
use crate::interrupt;
interrupt!(pit, unsafe {
    unsafe { PIT.add_tick() };

    EOI_pic1();
});

interrupt!(keyboard, unsafe {
    let scancode = nmk_utils::asm::inb(0x60);

    nmk_drivers::keyboard::key_handler(scancode);

    EOI_pic1();
});
