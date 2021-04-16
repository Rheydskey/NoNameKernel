#[cfg(feature = "stivale")]
pub mod stivale;

#[cfg(feature = "stivale2")]
pub mod stivale2;

pub mod start {
    #[cfg(feature = "stivale2")]
    pub mod stivale2 {
        use crate::bootloader::stivale2::{Stivale2Header, Stivale2Struct};
        use crate::{STACK, kmain};

        #[used]
        #[link_section = ".stivale2hdr"]
        static STIVALE2: Stivale2Header = Stivale2Header::new(STACK[0] as *const u8);

        #[no_mangle]
        pub unsafe extern "C" fn _start(stivale: &Stivale2Struct) -> ! {
            print!("{:?}", stivale.bootloader_brand);
            kmain();
            loop {}
        }
    }

    #[cfg(feature = "stivale")]
    pub mod stivale {
        use crate::bootloader::stivale::{StivaleStruct, StivaleHeader};
        use core::mem::transmute;
        use crate::{kmain, STACK};
        use core::lazy::OnceCell;
        use crate::drivers::vga::buffer::Writer;

        #[used]
        #[link_section = ".stivalehdr"]
        static STIVALEHDR: StivaleHeader = StivaleHeader::new(&STACK[0] as *const u8);

        #[no_mangle]
        pub extern "C" fn _start(stivale: &StivaleStruct) -> ! {

            let mut buffer = Writer::new_with_addr(stivale.framebuffer_addr);


            kmain();
            loop {
                buffer.write_byte(0xff)
            }
        }
    }
    #[cfg(feature = "bootimage")]
    mod bootimage {
        use crate::kmain;

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            kmain();
            loop {}
        }
    }
}
