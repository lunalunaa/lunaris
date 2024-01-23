fn main() {
    println!("cargo:rerun-if-changed=src/boot.S");
    println!("cargo:rerun-if-changed=src/boot_lab.S");
    println!("cargo:rerun-if-changed=linker.ld");
}
