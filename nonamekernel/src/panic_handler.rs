use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    let panic_message = _info.message().map_or_else(
        || "No Panic message",
        |args| args.as_str().unwrap_or("No Message Error"),
    );

    let location = if let Some(location) = _info.location() {
        location
    } else {
        print!("No location on panic");
        loop {}
    };

    println!(
        "PANIC at {}:{}:{} => {}",
        location.file(),
        location.line(),
        location.column(),
        panic_message
    );

    loop {}
}
