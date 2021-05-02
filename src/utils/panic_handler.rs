use crate::drivers::vga::buffer::Writer;
use crate::drivers::vga::vga_color::{Color, ColorCode};
use core::fmt::Write;
use core::panic::PanicInfo;
use crate::bootloader::BUFFER_ADDR;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut buffer = Writer::new_with_addr(unsafe {BUFFER_ADDR});

    let panic_message = if let Some(arg) = _info.message() {
        arg.as_str().unwrap_or("No Message Error")
    } else {"No Message Error"};
    let location = match _info.location() {
        Some(e) => e,
        None => {
            buffer.color_code = ColorCode::new(Color::Red, Color::Black);
            write!(buffer, "No Panic Location => {}", panic_message).unwrap();
            buffer.new_line();
            loop {}
        }
    };
    buffer.color_code = ColorCode::new(Color::LightGray, Color::Red);
    write!(
        buffer,
        "PANIC at {}:{}:{} => {}",
        location.file(),
        location.line(),
        location.column(),
        panic_message
    )
    .unwrap();
    buffer.new_line();
    loop {}
}
