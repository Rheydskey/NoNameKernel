use std::process::Command;
fn main() {
    // note: add error checking yourself.
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let mut git_hash = String::from_utf8(output.stdout).unwrap();
    git_hash = git_hash.split_at(7).0.to_string();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    cc::Build::new()
        .no_default_flags(true)
        .file("src/arch/x86_64/gdt.S")
        .pic(true)
        .static_flag(true)
        .shared_flag(false)
        .compile("x86_64_gdt");
}
