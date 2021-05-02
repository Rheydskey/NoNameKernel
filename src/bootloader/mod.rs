pub static mut BUFFER_ADDR: u64 = 0;

#[cfg(feature = "stivale2")]
pub mod stivale2_header;

#[cfg(feature = "grub")]
pub mod grub;

    #[cfg(feature = "stivale2")]
    pub mod stivale2 {
        use crate::{bootloader::stivale2_header::{Stivale2Header, Stivale2Struct}, kernel::kmain};
        use super::stivale2_header::Stivale2HeaderTagFramebuffer;

        static STACK: [u8; 4096] = [0; 4096];
        static FRAMEBUFFER_TAG: Stivale2HeaderTagFramebuffer = Stivale2HeaderTagFramebuffer::new();

        #[used]
        #[link_section = ".stivale2hdr"]
        static STIVALE2: Stivale2Header = Stivale2Header::new(STACK[0] as *const u8).set_tags((&FRAMEBUFFER_TAG as *const Stivale2HeaderTagFramebuffer).cast());
    }

    #[cfg(feature = "stivale")]
    pub mod stivale {
        use crate::{bootloader::stivale::{StivaleStruct, StivaleHeader}, drivers::vga::render::BUFFER};
        use crate::{kmain, STACK};
        use crate::drivers::vga::buffer::Writer;
        use crate::bootloader::BUFFER_ADDR;

        #[link_section = ".stivalehdr"]
        static STIVALEHDR: StivaleHeader = StivaleHeader::new(&STACK[0] as *const u8);

        #[no_mangle]
        pub extern "C" fn _start(stivale: &StivaleStruct) -> ! {
            unsafe {BUFFER_ADDR = stivale.framebuffer_addr};
            unsafe {BUFFER.get_or_init(|| Writer::new_with_addr(unsafe {BUFFER_ADDR}))};
            let buf = unsafe {BUFFER.get_mut().unwrap()};
            for _ in 0..1000 {
                buf.write_byte(0xFF)
            };
            //kmain();
            loop {}
        }
    }

    #[cfg(feature = "grub")]
    mod grub {
        use crate::kmain;
        use crate::drivers::vga::buffer::Writer;
        use crate::drivers::vga::render::BUFFER;
        use crate::bootloader::BUFFER_ADDR;
        use crate::bootloader::grub::{MultibootHeader, MULTIBOOT_HEADER_ARCHITECTURE_I386, MULTIBOOT2_HEADER_MAGIC};
        use core::mem::size_of;

        #[link_section = ".multiboot"]
        static HEADER: MultibootHeader = MultibootHeader {
            magic: MULTIBOOT2_HEADER_MAGIC,
            architecture: MULTIBOOT_HEADER_ARCHITECTURE_I386,
            header_lenght:  size_of::<MultibootHeader>() as u32,
            checksum: (u32::MAX - (0xe85250d6 + MULTIBOOT_HEADER_ARCHITECTURE_I386 + size_of::<MultibootHeader>() as u32)) as u32
        };

        #[no_mangle]
        pub fn _start() -> ! {
            unsafe { BUFFER_ADDR = 0xb8000 };
            unsafe { BUFFER.get_or_init(|| Writer::new_with_addr(0xb8000)) };
            kmain();
            loop {}
        }
    }
