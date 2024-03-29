use limine::{LimineBootInfoRequest, LimineMemmapRequest, LimineMemoryMapEntryType};
use nmk_archs::x86_64::{
    gdt::gdt_init,
    idt::init_idt,
    memory::{
        paging::{a, init_paging},
        pmm::PageMap4Level,
    },
    register::Cr3,
    stacktrace::StackTrace,
};
use nmk_drivers::{
    keyboard,
    serial::{init_serial, Com, Port},
};
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);
static MMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);
pub fn kmain() -> ! {
    nmk_status::Init::new("Serial").wait(init_serial);
    nmk_status::Init::new("GDT").wait(gdt_init);
    nmk_status::Init::new("IDT").wait(init_idt);
    nmk_archs::tests::sizeof();
    let mut memory_base_usable = 0;
    let mut memory_lenght = 0;
    for (mtype, base, lenght) in MMAP
        .get_response()
        .get()
        .unwrap()
        .memmap()
        .iter()
        .map(|f| (f.typ, f.base, f.len))
    {
        if mtype == LimineMemoryMapEntryType::Usable {
            memory_base_usable = base;
            memory_lenght = lenght;
        }
    }
    init_paging();
    println!("{:#X?}", a());

    let cr = Cr3::read();

    println!("{:#x}", cr.get_page_map());

    let port = Port::new(Com::COM1 as u16);
    loop {
        let value = port.read();
        if value != 0 {
            print!("{}", value);
            keyboard::key_handler(value);
        }
    }
}
