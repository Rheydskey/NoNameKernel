use std::process::Command;

fn main() {
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
}
