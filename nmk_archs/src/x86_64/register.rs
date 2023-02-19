use core::{arch::asm, fmt::Binary};

use bit_field::BitField;

#[derive(Debug)]
pub struct Cr3(u64);

impl Cr3 {
    pub fn read() -> Self {
        let value: u64;
        unsafe {
            asm!("mov {}, cr3", out(reg) value);
        }

        Self(value)
    }

    pub fn get_page_map(&self) -> u16 {
        self.0.get_bits(11..=32).try_into().unwrap()
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Binary for Cr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}
