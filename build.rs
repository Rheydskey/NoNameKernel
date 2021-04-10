use std::process::Command;

fn main() {
    // note: add error checking yourself.
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout)
        .unwrap()
        .split_at(7)
        .0
        .to_string();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    nasm_rs::Build::new()
        .target("x86_64-none-none")
        .file("src/arch/x86_64/gdt/gdt.asm")
        .flag("-F dwarf")
        .flag("-w+all")
        .flag("-Werror")
        .compile("x86_64_gdt")
        .expect("Error");
}
