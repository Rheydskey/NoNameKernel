use bitflags::bitflags;

pub const STIVALE2_STRUCT_TAG_PXE_SERVER_INFO: u64 = 0x29d1e96239247032;
pub const STIVALE2_HEADER_TAG_FRAMEBUFFER_ID: u64 = 0x3ecc1bc43d0f7971;
pub const STIVALE2_HEADER_TAG_FB_MTRR_ID: u64 = 0x4c7bb07731282e00;
pub const STIVALE2_HEADER_TAG_SMP_ID: u64 =  0x1ab015085f3273df;
pub const STIVALE2_HEADER_TAG_5LV_PAGING_ID:u64 = 0x932f477032007e8f;
pub const STIVALE2_BOOTLOADER_BRAND_SIZE: usize = 64;
pub const STIVALE2_BOOTLOADER_VERSION_SIZE: usize = 64;
pub const STIVALE2_STRUCT_TAG_CMDLINE_ID: u64 = 0xe5e76a1b4597a781;
pub const STIVALE2_STRUCT_TAG_MEMMAP_ID:u64 = 0x2187f79e8612de07;
pub const STIVALE2_STRUCT_TAG_FRAMEBUFFER_ID: u64 = 0x506461d2950408fa;
pub const STIVALE2_STRUCT_TAG_FB_MTRR_ID:u64 = 0x6bc1a78ebe871172;
pub const STIVALE2_STRUCT_TAG_MODULES_ID:u64 = 0x4b6fe466aade04ce;
pub const STIVALE2_STRUCT_TAG_RSDP_ID: u64 = 0x9e1786930a375e78;
pub const STIVALE2_STRUCT_TAG_EPOCH_ID: u64 = 0x566a7bed888e1407;
pub const STIVALE2_HEADER_TAG_FIRMWARE_ID: u64 = 0x359d837855e3858c;
pub const STIVALE2_STRUCT_TAG_SMP_ID: u64 = 0x34d1d96339647025;


bitflags! {
    pub struct Stivale2HeaderFlags: u64 {
        /// Set if the bootloader should apply kernel address space layout randomization
        const KASLR = 0x1;
    }
}


#[repr(C, packed)]
pub struct Stivale2Tag {
    pub identifier: u64,
    pub next: u64
}

impl Stivale2Tag {
    pub const fn new() -> Self {
        Self {
            identifier: 0,
            next: 0
        }
    }
}

#[repr(C)]
pub union Stivale2EntryPoint {
    function: extern "C" fn (stivale_struct_addr: usize) -> !,
    null: u64
}

#[repr(C, packed)]
pub struct Stivale2Header {
    entry_point: Stivale2EntryPoint,
    stack: *const u8,
    flags: Stivale2HeaderFlags,
    tags: *const ()
}

unsafe impl Sync for Stivale2Header {}
unsafe impl Send for Stivale2Header {}

impl Stivale2Header {
    pub const fn new(stack: *const u8) -> Self {
        Self {
            entry_point: Stivale2EntryPoint { null: 0 },
            stack,
            flags: Stivale2HeaderFlags::empty(),
            tags: core::ptr::null(),
        }
    }

    pub const fn set_entry_point(mut self, entry_point: extern "C" fn(stivale_struct_addr: usize) -> !) -> Self {
        self.entry_point = Stivale2EntryPoint {function: entry_point};
        self
    }

    pub const fn set_flags(mut self, flags: Stivale2HeaderFlags) -> Self {
        self.flags = flags;
        self
    }

    pub const fn set_tags(mut self, tag: *const ()) -> Self {
        self.tags = tag;
        self
    }
}

#[repr(C, packed)]
pub struct Stivale2HeaderTagFramebuffer {
    pub identifier: u64,
    pub next: *const (),
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_bpp: u16
}

unsafe impl Send for Stivale2HeaderTagFramebuffer {}
unsafe impl Sync for Stivale2HeaderTagFramebuffer {}

impl Stivale2HeaderTagFramebuffer {
    pub const fn new() -> Self {
        Self {
            identifier: STIVALE2_HEADER_TAG_FRAMEBUFFER_ID,
            next: core::ptr::null(),
            framebuffer_bpp: 0,
            framebuffer_height: 0,
            framebuffer_width: 0
        }
    }

    pub const fn resolution(mut self, width: u16, height: u16) -> Self {
        self.framebuffer_width = width;
        self.framebuffer_height = height;
        self
    }

    pub const fn bpp(mut self, bpp: u16) -> Self {
        self.framebuffer_bpp = bpp;
        self
    }

    pub const fn next(mut self, tag: *const ()) -> Self {
        self.next = tag;
        self
    }
}


#[repr(C, packed)]
pub struct Stivale2HeaderTagSmp {
    tag: Stivale2Tag,
    flags: u64,
}


pub struct Stivale2Struct {
    pub inner: *const Stivale2StructInner
}

#[repr(C, packed)]
pub struct Stivale2StructInner {
    pub bootloader_brand: [u8; STIVALE2_BOOTLOADER_BRAND_SIZE],
    pub bootloader_version: [u8; STIVALE2_BOOTLOADER_VERSION_SIZE],
    pub tags: u64
}

impl Stivale2Struct {
    pub fn get_inner(&self) -> &Stivale2StructInner {
        unsafe {&*self.inner}
    }

    pub fn get_tag(&self, identifier: u64) -> Option<u64> {
        let mut next: *const Stivale2Tag = self.get_inner().tags as *const Stivale2Tag;
        while !next.is_null() {
            let tag = unsafe {&*next};
            if tag.identifier == identifier {return Some(next as u64);}
            next = tag.next as *const Stivale2Tag;
        }

        None
    }

    pub fn get_framebuffer(&self) -> Option<&Stivale2StructTagFramebuffer> {
        self.get_tag(STIVALE2_STRUCT_TAG_FRAMEBUFFER_ID).map(|tag| unsafe {&*(tag as *const Stivale2StructTagFramebuffer)})
    }
}


#[repr(C, packed)]
pub struct Stivale2StructTagCmdline {
    tag: Stivale2Tag,
    cmdline: u64,
}


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

#[repr(C)]
pub enum Stivale2Fbuf {
    MmodelRGB = 1
}

#[repr(C, packed)]
pub struct Stivale2StructTagFramebuffer {
    pub identifier: u64,
    pub next: u64,
    pub framebuffer_addr: u64,
    pub framebuffer_width: u16,
    pub framebuffer_height: u16,
    pub framebuffer_pitch: u16,
    pub framebuffer_bpp: u16,
}

impl Stivale2StructTagFramebuffer {
    pub fn get_start_addr(&self) -> usize {
        self.framebuffer_addr as usize
    }

    pub fn get_end_add(&self) -> usize {
        self.framebuffer_addr as usize + self.size()
    }

    pub fn size(&self) -> usize {
        self.framebuffer_pitch as usize * self.framebuffer_height as usize * (self.framebuffer_bpp as usize / 8)
    }

    pub fn get_width(&self) -> u16 {
        self.framebuffer_width
    }

    pub fn get_height(&self) -> u16 {
        self.framebuffer_height
    }

    pub fn get_pitch(&self) -> u16 {
        self.framebuffer_pitch
    }

    pub fn get_bpp(&self) -> u16 {
        self.framebuffer_bpp
    }
}


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


#[repr(C, packed)]
pub struct Stivale2StructTagRspd {
    tag: Stivale2Tag,
    rspd: u64
}


#[repr(C, packed)]
pub struct Stivale2StructTagEpoch {
    tag: Stivale2Tag,
    epoch: u64
}


#[repr(C, packed)]
pub struct Stivale2StructTagFirmware {
    tag: Stivale2Tag,
    flags: u64
}


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

#[repr(C, packed)]
pub struct Stivale2StructTagPxeServerInfo {
    tag: Stivale2Tag,
    server_ip: u32
}