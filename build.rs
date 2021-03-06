extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::Builder;

fn main() {
    println!("cargo:rustc-link-lib=laszip_api");
    let bindings = Builder::default().header("wrapper.h").generate().expect(
        "Unable to generate bindings",
    );
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
