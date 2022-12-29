const PAGE_SIZE: u16 = 4096;

struct Region {
    start_addr: usize,
    lenght: u32,
    available: bool,
}

struct Pmm {}
