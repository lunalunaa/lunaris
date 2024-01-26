fn main() {
    println!("cargo:rerun-if-changed=src/asm/boot.S");
    println!("cargo:rerun-if-changed=src/asm/exception.S");
    println!("cargo:rerun-if-changed=src/asm/switch.S");
    println!("cargo:rerun-if-changed=linker.ld");
}
