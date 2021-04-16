

#[repr(C, packed)]
pub struct StivaleHeader {
    pub stack: *const u8,
    pub flags: u16,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_bpp: u16,
    pub entry_point: u64,
}

#[repr(C)]
pub union StivaleHeaderEntryPoint {
    function: extern "C" fn(stivale_struct_addr: usize) -> !,
    null: u64,
}

unsafe impl Send for StivaleHeader {}
unsafe impl Sync for StivaleHeader {}
impl StivaleHeader {
    pub const fn new(stack: *const u8) -> Self {
        Self {
            stack,
            flags: 0x0001,
            framebuffer_width: 10,
            framebuffer_height: 10,
            framebuffer_bpp: 1,
            entry_point: 0,
        }
    }
}

#[repr(C, packed)]
pub struct StivaleModule {
    begin: u64,
    end: u64,
    string: [u8; 128],
    next: u64,
}

#[repr(C)]
enum StivaleMmap {
    StivaleMmapUsable = 1,
    StivaleMmapReserved = 2,
    StivaleMmapAcpiReclaimable = 3,
    StivaleMmapAcpiNvs = 4,
    StivaleMmapBadMemory = 5,
    StivaleMmapKernelAndModules = 10,
    StivaleMmapBootloaderReclaimable = 0x1000,
    StivaleMmapFramebuffer = 0x1002,
}

#[repr(C, packed)]
pub struct StivaleMmapEntry {
    base: u64,
    lenght: u64,
    entry_type: u32,
    unused: u32,
}

const STIVALE_FBUF_MMODEL_RGB: u32 = 1;

#[repr(C, packed)]
pub struct StivaleStruct {
    pub cmdline: u64,
    pub memory_map_addr: u64,
    pub memory_map_entrie: u64,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u16,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_bpp: u16,
    pub rsdp: u64,
    pub module_count: u64,
    pub modules: u64,
    pub epoch: u64,
    pub flags: u64,

    pub fb_memory_model: u8,
    pub fb_red_mask_size: u8,
    pub fb_red_mask_shift: u8,
    pub fb_green_mask_size: u8,
    pub fb_green_mask_shift: u8,
    pub fb_blue_mask_size: u8,
    pub fb_blue_mask_shift: u8,
}

unsafe impl Send for StivaleStruct {}
unsafe impl Sync for StivaleStruct {}