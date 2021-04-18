pub static mut BUFFER_ADDR: u64 = 0;

#[cfg(feature = "stivale")]
pub mod stivale;

#[cfg(feature = "stivale2")]
pub mod stivale2;

pub mod start {
    #[cfg(feature = "stivale2")]
    pub mod stivale2 {
        use crate::bootloader::stivale2::{Stivale2Header, Stivale2Struct};
        use crate::{STACK, kmain};
        use crate::bootloader::BUFFER_ADDR;

        #[used]
        #[link_section = ".stivale2hdr"]
        static STIVALE2: Stivale2Header = Stivale2Header::new(STACK[0] as *const u8);

        #[no_mangle]
        pub fn _start(stivale: &Stivale2Struct) -> ! {
            crate::print!("{:?}", stivale.bootloader_brand);
            kmain();
            loop {}
        }
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
        pub fn _start(stivale: &StivaleStruct) -> ! {
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
    #[cfg(feature = "bootimage")]
    mod bootimage {
        use crate::kmain;
        use crate::drivers::vga::buffer::Writer;
        use crate::drivers::vga::render::BUFFER;
        use crate::bootloader::BUFFER_ADDR;

        #[no_mangle]
        pub fn _start() -> ! {
            unsafe  {BUFFER_ADDR = 0xb8000};
            unsafe {BUFFER.get_or_init(|| Writer::new_with_addr(0xb8000))};
            kmain();
            loop {}
        }
    }
}
