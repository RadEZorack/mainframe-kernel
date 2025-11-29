use std::env;
use std::path::Path;

fn main() {
    let entry_file = "src/arch/aarch64/entry.S";

    if !Path::new(entry_file).exists() {
        panic!("entry.S not found at {}", entry_file);
    }

    println!("cargo:rerun-if-changed={entry_file}");

    let target = env::var("TARGET").expect("TARGET not set by Cargo");

    let mut build = cc::Build::new();
    build.file(entry_file);
    build.target(&target);
    build.compile("entry");
}
