fn main() {
    println!("cargo:rustc-link-arg=--script=linker.ld");
    println!("cargo:rerun-if-changed=src/boot.S");
    println!("cargo:rerun-if-changed=src/boot_lab.S");
}
