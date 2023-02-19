struct PageTable(u64);
struct PageDirectoryTable(u64);

pub struct PageDirectoryPointer(u64);

pub struct PageMap4Level(u64);

impl PageMap4Level {
    pub fn new(pagemap: u64) -> Self {
        return Self(pagemap);
    }

    pub fn read(&self) {
        println!("\n\n{:x}", self.0);
    }

    pub fn raw(&self) -> u64 {
        return self.0;
    }

    pub fn page_directory_pointer(&self) -> PageDirectoryPointer {
        PageDirectoryPointer(self.0 & 0xFFFFFFFFFFFFFFFF)
    }
}
