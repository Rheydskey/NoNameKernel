use crate::{arch::x86_64::{gdt::gdt_init, idt::init_idt}, drivers::serial::{init_serial}, println_color, utils};
use crate::drivers::vga::vga_color::{Color, ColorCode};

pub fn kmain() {
    println_color!(
        ColorCode::new(Color::LightCyan, Color::Black),
        "Welcome on NoName Kernel {}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );

    utils::status::Init::new("Serial").wait(init_serial);
    utils::status::Init::new("GDT").wait(gdt_init);
    utils::status::Init::new("IDT").wait(init_idt);
}