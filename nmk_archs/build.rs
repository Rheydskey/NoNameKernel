fn main() {
    nasm_rs::Build::new()
        .target("x86_64-none-none")
        .file("src/x86_64/gdt/gdt.asm")
        .flag("-F dwarf")
        .flag("-w+all")
        .flag("-Werror")
        .compile("x86_64_gdt")
        .expect("Error");
}
