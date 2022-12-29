use core::arch::asm;

#[inline]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value);
    };
    value
}

#[inline]
pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
           "out dx, al",
           in("dx") port,
           in("al") value,
        );
    };
}

#[inline]
pub fn outl(port: u16, value: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
        );
    }
}

#[inline]
pub fn inl(port: u16) -> u32 {
    let ret: u32;
    unsafe {
        asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") ret,
        );
    }

    ret
}

#[inline]
pub fn wait() {
    outb(0x80, 0);
}

pub fn read_cr3() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) value);
    }

    value
}

pub fn read_ebp() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, rbp", out(reg) value);
    }

    value
}
