const MULTIBOOT_HEADER: u32 = 1;
const MULTIBOOT_SEARCH: u32 = 32768;
const MULTIBOOT_HEADER_ALIGN: u8 = 8;
const MULTIBOOT2_HEADER_MAGIC: u32 = 0xe85250d6;
const MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36d76289;
const MULTIBOOT_MOD_ALIGN: u32 = 0x00001000;
const MULTIBOOT_INFO_ALIGN: u32 = 0x00000008;
const MULTIBOOT_TAG_ALGIN: u32 = 8;
const MULTIBOOT_TAG_TYPE_END: u32 = 0;
const MULTIBOOT_TAG_TYPE_CMDLINE: u32 = 1;
const MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME: u32 = 2;
const MULTIBOOT_TAG_TYPE_MODULE: u32 = 3;
const MULTIBOOT_TAG_TYPE_BASIC_MEMINFO: u32 = 4;
const MULTIBOOT_TAG_TYPE_BASIC_BOOTDEV: u32 = 5;
const MULTIBOOT_TAG_TYPE_BASIC_MMAP: u32 = 6;
const MULTIBOOT_TAG_TYPE_BASIC_VBE: u32 = 7;
const MULTIBOOT_TAG_TYPE_BASIC_FRAMEBUFFER: u32 = 8;
const MULTIBOOT_TAG_TYPE_ELF_SECTIONS: u32 = 9;
const MULTIBOOT_TAG_TYPE_APM: u32 = 10;
const MULTIBOOT_HEADER_TAG_END: u32 = 0;
const MULTIBOOT_HEADER_TAG_INFORMATION_REQUEST: u32 = 0;
const MULTIBOOT_HEADER_TAG_INFORMATION_ADDRESS: u32 = 0;
const MULTIBOOT_HEADER_TAG_ENTRY_ADDRESS: u32 = 0;
const MULTIBOOT_HEADER_TAG_CONSOLE_FLAGS: u32 = 0;
const MULTIBOOT_HEADER_TAG_FRAMEBUFFER: u32 = 0;
const MULTIBOOT_HEADER_TAG_MODULE_ALIGN: u32 = 0;
const MULTIBOOT_HEADER_ARCHITECTURE_I386: u32 = 0;
const MULTIBOOT_HEADER_ARCHITECTURE_MIPS32: u32 = 0;
const MULTIBOOT_HEADER_TAG_OPTIONAL: u32 = 0;
const MULTIBOOT_CONSOLE_FLAGS_CONSOLE_REQUIRED: u32 = 0;
const MULTIBOOT_CONSOLE_FLAGS_EGA_TEXT_SUPPORTED: u32 = 0;
const MULTIBOOT_FRAMEBUFFER_TYPE_INDEXED: u32 = 0;
const MULTIBOOT_FRAMEBUFFER_TYPE_RGB: u32 = 1;
const MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT: u32 = 2;

struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_lenght: u32,
    checksum: u32
}

struct MultibootHeaderTag {
    tag_type: u16,
    flags: u16,
    size: u32
}

struct MultibootHeaderTagInformationRequest {
    information_type: u16,
    flags: u16,
    size: u16,
    request: [u32; 0]
}

struct MultibootHeaderTagAddress {
    address_type: u16,
    flags: u16,
    size: u32,
    header_addr: u32,
    load_addr: u32,
    load_end_addr: u32,
    bss_end_addr: u32
}

struct MultibootHeaderTagEntryAddress {
    address_type: u16,
    flags: u16,
    size: u32,
    entry_addr: u32
}

struct MultibootHeaderTagFramebuffer {
    framebuffer_type : u16,
    flags: u16,
    size: u32,
    width: u32,
    height: u32,
    detph: u32
}

struct MultibootHeaderTagModuleAlign {
    module_type: u16,
    flags: u16,
    size: u32,
    width: u32,
    height: u32,
    detph: u32,
}

#[derive(Clone, Copy)]
struct MultibootColor {
    red: u8,
    green: u8,
    blue: u8
}

#[repr(C, packed)]
struct MultibootMmapEntry {
    addr: u64,
    len: u64,
    zero: u32
}

type MultibootMemoryMap = MultibootMmapEntry;

struct MultibootTag {
    tag_type: u32,
    size: u32
}

struct MultibootTagString {
    tag_type: u32,
    size: u32,
    string: [u32; 0]
}

struct MultibootTagModule {
    module_type: u32,
    size: u32,
    mod_start: u32,
    mod_end: u32,
    cmdline: [u8;0]
}

struct MultibootTagBasicMeminfo {
    meminfo_type: u32,
    size: u32,
    mem_lower: u32,
    mem_upper: u32,
}

struct MultibootTagBootdev {
    tage_type: u32,
    size: u32,
    biosdev: u32,
    slice: u32,
    part: u32
}

struct MultibootTagMmap {
    mmap_type: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    entries: [MultibootMmapEntry; 0]
}

struct MultibootVbeInfoBlock {
    external_specification: [u8;512]
}


struct MultibootVbeModeInfoBlock {
    external_specification: [u8;256]
}

struct MultibootTagVBE {
    vbe_type: u32,
    vbe_size: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
    vbe_control_info: MultibootVbeInfoBlock,
    vbe_mode_info: MultibootVbeModeInfoBlock
}

struct MultibootTagFramebufferCommon {
    tagframebuffer_type: u32,
    size: u32,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    reserved: u16
}

impl MultibootTagFramebufferCommon {
    const MULTIBOOT_FRAMEBUFFER_TYPE_INDEXED: u32 = MULTIBOOT_FRAMEBUFFER_TYPE_INDEXED;
    const MULTIBOOT_FRAMEBUFFER_TYPE_RGB: u32 = MULTIBOOT_FRAMEBUFFER_TYPE_RGB;
    const MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT: u32 = MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT;
}

#[derive(Clone, Copy)]
pub struct MultibootFramebufferPalette {
    num_colors: u16,
    palette: MultibootColor,
}

#[derive(Clone, Copy)]
pub struct MultibootFramebufferMaskField {
    framebuffer_red_field_position: u8,
    framebuffer_red_mask_size: u8,
    framebuffer_green_field_position: u8,
    framebuffer_green_mask_size: u8,
    framebuffer_blue_field_position: u8,
    framebuffer_blue_mask_size: u8,
}

union MultibootFramebufferOnion {
    framebuffer: MultibootFramebufferPalette,
    framebuffer_palette: MultibootFramebufferMaskField
}

struct MultibootTagFramebuffer {
    common: MultibootTagFramebufferCommon,
    framebuffer: MultibootFramebufferOnion
}

struct MultibootTagElfSections {
    elf_type: u32,
    size: u32,
    num: u32,
    entsize: u32,
    shndx: u32,
    sections: [u8; 0]
}

struct MultibootTagApm {
    apm_type: u32,
    size: u32,
    version: u32,
    cseg: u32,
    offset: u32,
    cseg_16: u16,
    dseg: u16,
    flags: u16,
    cseg_len: u16,
    cseg_16_len: u16,
    dseg_len: u16
}
