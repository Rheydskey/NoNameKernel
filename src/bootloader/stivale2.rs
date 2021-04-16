#[repr(C, packed)]
pub struct Stivale2Tag {
    pub identifier: u64,
    pub next: u64
}

#[repr(C, packed)]
pub struct Stivale2Header {
    entry_point: u64,
    stack: *const u8,
    flags: u64,
    tags: *const ()
}

unsafe impl Sync for Stivale2Header {}

impl Stivale2Header {
    pub const fn new(stack: *const u8) -> Self {
        Self {
            entry_point: 0,
            stack,
            flags: 0,
            tags: core::ptr::null(),
        }
    }
}

pub const STIVALE2_HEADER_TAG_FRAMEBUFFER_ID: u64 = 0x3ecc1bc43d0f7971;
pub const STIVALE2_HEADER_TAG_FB_MTRR_ID: u64 = 0x4c7bb07731282e00;

#[repr(C, packed)]
pub struct Stivale2HeaderTagFramebuffer {
    pub tag: Stivale2Tag,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_bpp: u16
}

pub const STIVALE2_HEADER_TAG_SMP_ID: u64 =  0x1ab015085f3273df;

#[repr(C, packed)]
pub struct Stivale2HeaderTagSmp {
    tag: Stivale2Tag,
    flags: u64,
}

pub const STIVALE2_HEADER_TAG_5LV_PAGING_ID:u64 = 0x932f477032007e8f;


pub const STIVALE2_BOOTLOADER_BRAND_SIZE: usize = 64;
pub const STIVALE2_BOOTLOADER_VERSION_SIZE: usize = 64;

#[repr(C, packed)]
pub struct Stivale2Struct {
    pub bootloader_brand: [char; STIVALE2_BOOTLOADER_BRAND_SIZE],
    bootloader_version: [char; STIVALE2_BOOTLOADER_VERSION_SIZE],
    tags: u64
}

pub const STIVALE2_STRUCT_TAG_CMDLINE_ID: u64 = 0xe5e76a1b4597a781;

#[repr(C, packed)]
pub struct Stivale2StructTagCmdline {
    tag: Stivale2Tag,
    cmdline: u64,
}

pub const STIVALE2_STRUCT_TAG_MEMMAP_ID:u64 = 0x2187f79e8612de07;

#[repr(C)]
pub enum Stivale2Mmap {
    USABLE = 1,
    RESERVED = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadMemory = 5,
    BootloaderReclaimable = 0x1000,
    KernelAndModules = 0x1001
}

#[repr(C, packed)]
pub struct Stivale2MmapEntry {
    base: u64,
    lenght: u64,
    entry_type: u32,
    unused: u32
}

#[repr(C, packed)]
pub struct Stivale2StructTagMemmap {
    tag: Stivale2Tag,
    entries: u64,
    memmap: [Stivale2MmapEntry]
}

pub const STIVALE2_STRUCT_TAG_FRAMEBUFFER_ID: u64 = 0x506461d2950408fa;

#[repr(C)]
pub enum Stivale2Fbuf {
    MmodelRGB = 1
}

#[repr(C, packed)]
pub struct Stivale2StructTagFramebuffer {
    tag: Stivale2Tag,
    framebuffer_addr: u64,
    framebuffer_width: u16,
    framebuffer_height: u16,
    framebuffer_pitch: u16,
    framebuffer_bpp: u16,
    memory_model: u8,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
}

pub const STIVALE2_STRUCT_TAG_FB_MTRR_ID:u64 = 0x6bc1a78ebe871172;
pub const STIVALE2_STRUCT_TAG_MODULES_ID:u64 = 0x4b6fe466aade04ce;

#[repr(C, packed)]
pub struct Stivale2Module {
    begin: u64,
    end: u64,
    string: [char; 128]
}

#[repr(C, packed)]
pub struct Stivale2StructTagModules {
    tag: Stivale2Tag,
    module_count: u64,
    modules: [Stivale2Module]
}

pub const STIVALE2_STRUCT_TAG_RSDP_ID: u64 = 0x9e1786930a375e78;

#[repr(C, packed)]
pub struct Stivale2StructTagRspd {
    tag: Stivale2Tag,
    rspd: u64
}

pub const STIVALE2_STRUCT_TAG_EPOCH_ID: u64 = 0x566a7bed888e1407;

#[repr(C, packed)]
pub struct Stivale2StructTagEpoch {
    tag: Stivale2Tag,
    epoch: u64
}

pub const STIVALE2_HEADER_TAG_FIRMWARE_ID: u64 = 0x359d837855e3858c;

#[repr(C, packed)]
pub struct Stivale2StructTagFirmware {
    tag: Stivale2Tag,
    flags: u64
}

const STIVALE2_STRUCT_TAG_SMP_ID: u64 = 0x34d1d96339647025;

#[repr(C, packed)]
pub struct Stivale2SmpInfo {
    processor_id: u32,
    lapic_id: u32,
    target_stack: u64,
    goto_address: u64,
    extra_argmunent: u64
}

#[repr(C, packed)]
pub struct Stivale2StructTagSmp {
    tag: Stivale2Tag,
    flags: u64,
    bsp_lapic_id: u32,
    unused: u32,
    cpu_count: u64,
    smp_info: [Stivale2SmpInfo]
}

pub const STIVALE2_STRUCT_TAG_PXE_SERVER_INFO: u64 = 0x29d1e96239247032;

#[repr(C, packed)]
pub struct Stivale2StructTagPxeServerInfo {
    tag: Stivale2Tag,
    server_ip: u32
}