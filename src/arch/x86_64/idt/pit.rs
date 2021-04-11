use crate::utils::asm::{outb, wait};

pub const PIT_FREQ: u64 = 1193182;
pub static mut PIT: PIT = PIT::default();

#[derive(Debug)]
pub struct PIT {
    pub time_since_last: u64,
    pub divisor: u64
}

impl PIT {
    #[inline]
    pub const fn default() ->  Self {
        Self {
            time_since_last: 0,
            divisor: 65535
        }
    }

    #[inline]
    pub fn add_tick(&mut self) {
        self.time_since_last += 1;
    }

    #[inline]
    pub fn get_freq(&self) -> u64 {
        PIT_FREQ / self.divisor
    }

    #[inline]
    pub unsafe fn set_divisor(&mut self, divisor: u64) {
        outb(0x40, (divisor & 0xFF) as u8);
        wait();

        outb(0x40, ((divisor >> 8) & 0xFF ) as u8);
        wait();
    }
}

pub fn initpit() {
    unsafe {
        PIT.set_divisor(1193180);
    }
}