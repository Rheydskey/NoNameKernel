use core::mem::size_of;

#[derive(Clone, Copy)]
pub struct GDT {
    GDT: [GDTEntry; 5]
}

impl GDT {
    pub fn new() -> Self {
        Self {
            GDT: [GDTEntry::new(0,0); 5]
        }
    }
    pub fn zero(&mut self) {
        self.GDT[0] = GDTEntry::new(0,0);
    }
    pub fn set(&mut self,index: usize, entry: GDTEntry) {
        self.GDT[index] = entry;
    }
}

#[derive(Default, Clone, Copy)]
#[repr(packed)]
pub struct GDTPointer {
    len: u16,
    address: u64
}
impl GDTPointer {
    pub unsafe fn register(&mut self, gdt: GDT) {
        self.len = size_of::<GDT>() as u16;
        self.address = &gdt as *const _ as u64;
    }
}
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    flags: u8,
    granularity: u8,
    base_high: u8,
}

impl GDTEntry {
    pub fn new(flag: u8, granularity: u8) -> Self {
        Self {
            base_high: 0,
            base_mid: 0,
            base_low: 0,
            flags: flag | GDTFlags::PRESENT as u8,
            granularity: (granularity << 4) | 0x0F,
            limit_low: 0
        }
    }
}

pub enum GDTSelector {
    NullSelector = 0,
    KernelCode = 0x8,
    KernelData = 0x10,
    UserData = 0x1b,
    UserCode = 0x23,
}

pub enum GDTFlags {
    WRITABLE = 0b10,
    USER = 0b1100000,
    PRESENT = 0b10000000,
    DS = 0b10000,
    CS = 0b11000,
}

pub enum GDTGranularity {
    LongModeGranularity = 0x2,
}