fn main() {
    println!("cargo:rerun-if-changed=src/boot.S");
    println!("cargo:rerun-if-changed=src/boot_lab.S");
    println!("cargo:rerun-if-changed=src/boot_alt.S");
    println!("cargo:rerun-if-changed=src/exception.S");
    println!("cargo:rerun-if-changed=src/switch.S");
    println!("cargo:rerun-if-changed=linker.ld");
}
