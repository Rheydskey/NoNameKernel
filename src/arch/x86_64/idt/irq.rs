use super::{pit::PIT, EOI_pic1};
use crate::{interrupt, utils};
interrupt!(pit, unsafe {
    unsafe { PIT.add_tick() };

    EOI_pic1();
});

interrupt!(keyboard, unsafe {
    let scancode = utils::asm::inb(0x60);

    crate::drivers::keyboard::key_handler(scancode);

    EOI_pic1();
});
