fn main() {
    // Migrations...
    println!("cargo:rerun-if-changed=migrations");
}
