//extern crate bindgen;
//
//use std::env;
//use std::path::PathBuf;
//
fn main() {
//    // tell cargo to tell rustc to link the system jack shared library
    println!("cargo:rustc-link-lib=jack");
//
//    // The bindgen::Builder is the main entry point to bindgen, and lets
//    // you build up options for the resulting bindings.
//    let bindings = bindgen::Builder::default()
//    // Do not generate unstable Rust code that
//    // requires a nightly rustc and enabling unstable features.
//    .unstable_rust(false)
//    // the input header we would like to generate bindings for
//    .header("jack.h")
//    // finish the builder and generate the bindings
//    .generate()
//    // unwrap the result and panic on failure.
//    .expect("Unable to generate bindings");
//
//    // write the bindings to the $OUT_DIR/jack.rs file.
//    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//    bindings.write_to_file(out_path.join("jack.rs"))
//        .expect("Couldn't write bindings!");
}