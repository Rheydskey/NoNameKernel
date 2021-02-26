use std::process::Command;
fn main() {
    // note: add error checking yourself.
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let mut git_hash = String::from_utf8(output.stdout).unwrap();
    git_hash = git_hash.split_at(7).0.to_string();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}