const GDT_ENTRIES: usize = 6;

static mut GDT: [GDTEntry; GDT_ENTRIES] = [GDTEntry::null(); GDT_ENTRIES];

#[repr(C, packed)]
pub struct GDTPtr {
    limit: u16,
    base: u64,
}

impl GDTPtr {
    #[inline]
    pub const fn new(limit: u16, base: u64) -> Self {
        Self { limit, base }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    flag: u8,
    granularity: u8,
    base_high: u8,
}

impl GDTEntry {
    #[inline]
    pub const fn new(
        limit_low: u16,
        base_low: u16,
        base_middle: u8,
        flag: u8,
        granularity: u8,
        base_high: u8,
    ) -> Self {
        Self {
            limit_low,
            base_low,
            base_middle,
            flag,
            granularity,
            base_high,
        }
    }

    #[inline]
    const fn null() -> Self {
        Self::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00)
    }
}

#[link(name = "x86_64_gdt")]
extern "C" {
    fn load_gdt(gdt_descriptor: *const GDTPtr);
}

pub fn gdt_init() -> bool {
    unsafe {
        GDT[0] = GDTEntry::new(0, 0, 0, 0x00, 0x00, 0);
        GDT[1] = GDTEntry::new(0, 0, 0, 0x9A, 0xA0, 0);
        GDT[2] = GDTEntry::new(0, 0, 0, 0x92, 0xA0, 0);
        GDT[3] = GDTEntry::new(0, 0, 0, 0xFA, 0xA0, 0);
        GDT[4] = GDTEntry::new(0, 0, 0, 0xF2, 0xA0, 0);

        let gdt_descriptor = GDTPtr::new(
            (core::mem::size_of::<[GDTEntry; GDT_ENTRIES]>() - 1) as u16,
            (&GDT as *const _) as u64,
        );

        load_gdt(&gdt_descriptor as *const _);

        true
    }
}
