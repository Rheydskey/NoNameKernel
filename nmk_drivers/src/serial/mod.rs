use core::fmt::{self, Write};

use nmk_utils::asm::{inb, outb};

pub struct Port {
    port: u16,
}

impl Port {
    pub fn new(port: u16) -> Port {
        Self { port }
    }

    pub fn read(&self) -> u8 {
        inb(self.port)
    }
    pub fn write(&self, value: u8) {
        outb(self.port, value);
    }
}

impl fmt::Write for Port {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_serial_str(s);
        Ok(())
    }
}

#[repr(C)]
pub enum Com {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
}
const PORT: u16 = Com::COM1 as u16;

pub fn init_serial<'a>() -> Result<u8, &'a str> {
    let com = Com::COM1 as u16;

    outb((com + 1), 0x00);
    outb((com + 3), 0x80);
    outb((com), 0x3);
    outb((com + 1), 0);
    outb((com + 3), 0x03);
    outb((com + 2), 0xC7);
    outb((com + 4), 0x0B);
    outb((com + 4), 0x1E);
    outb((com), 0xAE);

    if inb(com) != 0xAE {
        return Err("Serial is faulty");
    }

    outb(PORT + 4, 0x0F);

    Ok(0)
}

pub fn serial_received() -> u8 {
    inb(PORT + 5) & 1
}

pub fn read_serial() -> u8 {
    while serial_received() == 0 {}

    inb(PORT)
}

pub fn is_transmit_empty() -> u8 {
    inb(PORT + 5) & 0x20
}

#[inline]
pub fn write_serial_bytes(bytes: &[u8]) {
    while is_transmit_empty() == 0 {}
    for i in bytes {
        outb(PORT, *i);
    }
}

#[inline]
pub fn write_serial(a: char) {
    while is_transmit_empty() == 0 {}

    outb(PORT, a as u8);
}

#[inline]
pub fn write_serial_str(a: &str) {
    for i in a.as_bytes() {
        write_serial(*i as char);
    }
}

#[inline]
pub fn _print(args: fmt::Arguments<'_>) {
    Port::new(Com::COM1 as u16).write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {{$crate::serial::_print(format_args_nl!($($arg)*));}};
}
