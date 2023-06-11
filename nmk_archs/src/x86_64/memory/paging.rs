use core::ops::{Index, IndexMut};

use bit_field::BitField;

pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    pub const fn truncate(&self) -> Self {
        Self((self.0 << 16 as i64) >> 16 as u64)
    }
    pub const fn is_align(&self, align: u64) -> bool {
        self.0 == self.align(align)
    }

    pub const fn align(&self, align: u64) -> u64 {
        assert!(align.is_power_of_two(), "Must be a power of two");
        self.truncate().0 & (!align - 1)
    }
}

struct VirtualAddress(u64);

impl VirtualAddress {
    pub fn get_pmlx_offset<const T: u8>(&self) -> u64 {
        PMLX::<T>::get_index(self.0)
    }

    pub const fn truncate(&self) -> Self {
        Self((self.0 << 16 as i64) >> 16 as u64)
    }

    pub const fn is_align(&self, align: u64) -> bool {
        self.0 == self.align(align)
    }

    pub const fn align(&self, align: u64) -> u64 {
        assert!(align.is_power_of_two(), "Must be a power of two");
        self.truncate().0 & (!align - 1)
    }
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct PmlEntry(u64);

impl PmlEntry {
    const PHYSADDR: u64 = 0x000ffffffffff000;
    const FLAGMASK: u64 = 0xFFF;
    const PRESENT: u64 = (1 << 0);
    const WRITE: u64 = (1 << 1);
    const USER: u64 = (1 << 2);
    const WRITE_THROUGH: u64 = (1 << 3);
    const NO_CACHE: u64 = (1 << 4);
    const ACCESSED: u64 = (1 << 5);
    const DIRTY: u64 = (1 << 6);
    const HUGE_PAGE: u64 = (1 << 7);

    pub const fn is_set<const T: u32>(&self, flags: u64) -> bool {
        self.0 & flags != 0
    }

    /// PmlEntry schema:
    ///
    ///
    ///
    ///
    pub fn new(physical_addr: PhysicalAddress, flags: u64) -> Self {
        println!("Physical : {:#x}", physical_addr.0 >> 12);
        let mut pml: u64 = 0;
        pml.set_bits(0..=7, flags);
        pml.set_bits(8..=11, 0);
        pml.set_bits(12..=51, 51 << physical_addr.0 >> 12);
        pml.set_bits(52..=63, 0);
        println!("{:#x}", pml);
        Self(pml)
    }
}

#[repr(C)]
pub struct PMLX<const LEVEL: u8> {
    entries: [PmlEntry; 512],
}

impl<const LEVEL: u8> PMLX<LEVEL> {
    pub fn get_index(addr: u64) -> u64 {
        (addr >> (12 + LEVEL * 9)) & 0x1ff
    }

    pub fn new() -> Self {
        PMLX {
            entries: [PmlEntry::new(PhysicalAddress(0), PmlEntry::WRITE); 512],
        }
    }
}

impl<const LEVEL: u8> Index<usize> for PMLX<LEVEL> {
    type Output = PmlEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<const LEVEL: u8> IndexMut<usize> for PMLX<LEVEL> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

type PML4 = PMLX<4>;
type PML3 = PMLX<3>;
type PML2 = PMLX<2>;
type PML1 = PMLX<1>;

pub fn a() -> (u64, u64, u64, u64, u64) {
    println!("PADDR : {:b}", PmlEntry::PHYSADDR);
    println!(": {:#b}", PmlEntry::PRESENT);
    let addre = VirtualAddress(0xffffffff80000000);
    (
        PML4::get_index(0xffffffff80000000),
        PML3::get_index(0xffffffff80000000),
        PML2::get_index(0xffffffff80000000),
        addre.get_pmlx_offset::<1>(),
        addre.get_pmlx_offset::<0>(),
    )
}

pub fn init_paging() {
    let mut pml1 = PML1::new();
    let pml2 = PML2::new();

    PhysicalAddress(&pml2 as *const _ as u64).is_align(4096);
    pml1[0] = PmlEntry::new(
        PhysicalAddress(&pml2 as *const _ as u64),
        PmlEntry::PRESENT | PmlEntry::WRITE,
    );
}
