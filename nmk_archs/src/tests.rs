use core::mem::size_of;

use crate::x86_64::memory::paging::{PmlEntry, PMLX};

pub fn sizeof() {
    if size_of::<PmlEntry>() != 8 {
        println!("Error: PmlEntry size is {}", size_of::<PmlEntry>());
        panic!("test wrong");
    }

    if size_of::<PMLX<1>>() != 4096 {
        println!("Pmlx size is {}", size_of::<PMLX<1>>())
    }
}

#[macro_export]
macro_rules! assert_panic {
    ($value:expr, $eq:expr, $panic_message:expr, $( $args:expr ), *) => {
        if ($value != $eq) {
            panic!($panic_message, $($args,)* )
        }
    };
}
