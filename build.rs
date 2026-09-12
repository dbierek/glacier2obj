extern crate cbindgen;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&crate_dir).join("generated");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Failed to find or parse cbindgen.toml");

    fs::create_dir_all(&out_dir).expect("Failed to create output directory for headers");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("navkit-rpkg-lib.h"));
}