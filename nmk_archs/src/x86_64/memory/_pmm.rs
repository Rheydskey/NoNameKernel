const ENTRY_COUNT: usize = 16;

pub static mut PAGE_DIRECTORY: PageTable = PageTable::new();
pub static mut NEXT_FREE_MAP: u32 = 0;


#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    #[inline]
    pub const fn new() -> Self {
        const EMPTY: PageTableEntry = PageTableEntry::new();
        PageTable {
            entries: [EMPTY; ENTRY_COUNT],
        }
    }

    #[inline]
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.entries.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.entries.iter_mut()
    }

}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PageTableEntry {
    pub entry: u64,
}

impl PageTableEntry {
    #[inline]
    pub const fn new() -> Self {
        PageTableEntry { entry: 0 }
    }

    #[inline]
    pub const fn is_unused(&self) -> bool {
        self.entry == 0
    }

    #[inline]
    pub const fn set_unused(&mut self) {
        self.entry = 0;
    }

    #[inline]
    pub fn addr(&self) -> u64 {
        self.entry & 0x000f_ffff_ffff_f000
    }
    pub fn set(&mut self, addr: u64) {
        self.entry = addr;
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut f = f.debug_struct("PageTableEntry");
        f.field("addr", &format_args!("{:#X}", self.addr()));
        if self.is_unused() {
            f.field("Used", &false);
        } else {
            f.field("Used", &true);
        }
        f.finish()
    }
}