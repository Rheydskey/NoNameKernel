use std::env;
use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout)
        .unwrap()
        .split_at(7)
        .0
        .to_string();
    let kernel_name = env::var("CARGO_PKG_NAME").unwrap();

    // Tell rustc to pass the linker script to the linker.
    println!("cargo:rustc-link-arg-bin={kernel_name}=--script=.cargo/linker.ld");
    println!("cargo:rustc-env=GIT_HASH={git_hash}");
}
