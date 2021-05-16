use crate::utils::asm::{inb, outb};

struct Port {
    port: u16
}

impl Port {
    pub fn new(port: u16) -> Port {
        Self {
            port
        }
    }

    pub fn read(&mut self) -> u8 {
        inb(self.port)
    }
    pub fn write(&mut self, value: u8) {
        outb(self.port, value);
    }
}

#[repr(C)]
enum Com {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8
}
const PORT: u16 = Com::COM1 as u16;


pub fn init_serial() -> u8 {
    let com = Com::COM1 as u32;

    outb((com + 1) as u16, 0x00);
    outb((com + 3) as u16, 0x80);
    outb((com) as u16, 0x3);
    outb((com + 1) as u16, 0);
    outb((com + 3) as u16, 0x03);
    outb((com + 2) as u16, 0xC7);
    outb((com + 4) as u16, 0x0B);
    outb((com + 4) as u16, 0x01);

    if inb(PORT + 0) != 0xAE {
        return 1;
    }

    outb(PORT + 4, 0x0F);
    return 0;
}

pub fn serial_received() -> u8 {
    return inb(PORT + 5) & 1;
}

pub fn read_serial() -> u8 {
    while serial_received() == 0 {}

    return inb(PORT);
}

pub fn is_transmit_empty() -> u8 {
   return inb(PORT + 5) & 0x20;
}

pub fn write_serial(a: char) {
    while is_transmit_empty() == 0 {}

    outb(PORT, a as u8);
}
