use nmk_archs::x86_64::{gdt::gdt_init, idt::init_idt};
use nmk_drivers::serial::init_serial;

pub fn kmain() {
    nmk_status::Init::new("Serial").wait(init_serial);
    nmk_status::Init::new("GDT").wait(gdt_init);
    nmk_status::Init::new("IDT").wait(init_idt);
}
