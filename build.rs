fn main() {
    println!("cargo:rerun-if-changed=src/boot.S");
}
