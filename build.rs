fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/asm/entry.S");

    cc::Build::new()
        .file("src/arch/aarch64/asm/entry.S")
        .target("aarch64-unknown-none")
        .flag("-march=armv8-a")
        .compile("entry");
}
