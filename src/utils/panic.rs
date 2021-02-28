use core::panic::PanicInfo;
use crate::lib::vga::Writer;
use crate::lib::vga_color::{Color, ColorCode};
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut buffer = Writer::default();
    let panic_message = match _info.message() {
        Some(arg) => arg.as_str().unwrap_or("No Message Error"),
        None => "No message Error",
    };
    let location = match _info.location() {
        Some(e) => e,
        None => {
            buffer.color_code = ColorCode::new(Color::Red, Color::Black);
            write!(buffer, "No Panic Location => {}", panic_message).unwrap();
            buffer.new_line();
            loop {};
        }
    };
    buffer.color_code = ColorCode::new(Color::LightGray, Color::Red);
    write!(
        buffer,
        "PANIC at {}:{}:{} => {}",
        location.file(), location.line(), location.column(), panic_message
    )
    .unwrap();
    buffer.new_line();
    loop {}
}