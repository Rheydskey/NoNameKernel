use crate::drivers::vga::buffer::Writer;
use core::fmt::{self, Write};
use core::lazy::OnceCell;

pub static mut RENDER: OnceCell<Writer> = OnceCell::new();

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe {
        if let Some(e) = RENDER.get_mut() {
            e.write_fmt(args).expect("Error");
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga::render::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", $crate::file!(), $crate::line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
