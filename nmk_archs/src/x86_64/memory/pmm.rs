use core::usize;

const BITMAP_SIZE: usize = 2048;
static mut BITMAP: &mut [u8] = &mut [0; BITMAP_SIZE];
static mut MEMORY_END: u64 = 0xFFFF;
static mut NEXT_FREE_PAGE: i32 = 0;

pub fn alloc_page(page_count: u64) {}

pub fn free_page(page_addr: &(), page_count: u64) {}

pub fn init_pmm() {}

pub fn get_bitmap_array_index(page_addr: u64) -> u64 {
    page_addr / 8
}

pub fn get_bitmap_bit_index(page_addr: u64) -> u64 {
    page_addr % 8
}

pub fn bitmap_set_bit(page_addr: u64) {
    let bit = get_bitmap_bit_index(page_addr);
    let byte = get_bitmap_array_index(page_addr);

    unsafe {
        BITMAP[byte as usize] |= 1 << bit;
    }
}

pub fn bitmap_clear_bit(page_addr: u64) {
    let bit = get_bitmap_bit_index(page_addr);
    let byte = get_bitmap_bit_index(page_addr);
    unsafe { BITMAP[byte as usize] &= !(1 << bit) };
}
