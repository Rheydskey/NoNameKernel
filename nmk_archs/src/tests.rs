use core::mem::size_of;

use crate::x86_64::memory::pmm::PageMapLevelEntry;

#[test]
pub fn sizeofpml() {
    assert!(size_of::<PageMapLevelEntry>() == 64)
}
