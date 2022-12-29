use nmk_archs::x86_64::{gdt::gdt_init, idt::init_idt};
use nmk_drivers::{
    keyboard,
    serial::{init_serial, Com, Port},
};
use stivale_boot::v2::StivaleStruct;

pub fn kmain(_ptr: &'static StivaleStruct) -> ! {
    nmk_status::Init::new("Serial").wait(init_serial);
    nmk_status::Init::new("GDT").wait(gdt_init);
    nmk_status::Init::new("IDT").wait(init_idt);

    let port = Port::new(Com::COM1 as u16);
    loop {
        let value = port.read();
        if value != 0 {
            print!("{}", value);
            keyboard::key_handler(value);
        }
    }
}
