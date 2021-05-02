use crate::{arch::x86_64::{gdt::gdt_init, idt::init_idt}, println_color, utils};
use crate::drivers::vga::vga_color::{Color, ColorCode};

pub fn kmain() {
    println_color!(
        ColorCode::new(Color::LightCyan, Color::Black),
        "Welcome on NoName Kernel {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );
    let mut e = utils::status::Init::new("Test");
    e.pending();
    e.ok();
    utils::status::Init::new("GDT").wait(gdt_init);
    utils::status::Init::new("IDT").wait(init_idt);
}