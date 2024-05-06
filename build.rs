use style4rs::Builder;

fn main() {
    // Style4rs...
    let out_file = std::path::Path::new("target/site/pkg/cribbage-components.css");
    Builder::new()
        .using_out_file(out_file)
        .build().ok();

    // Migrations...
    println!("cargo:rerun-if-changed=migrations");
}
