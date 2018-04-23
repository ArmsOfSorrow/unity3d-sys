extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()

    .header("wrapper.h")

    .clang_arg("-IC:/Program Files/Unity/Hub/Editor/2017.4.1f1/Editor/Data/PluginAPI")

    .generate()

    .expect("couldn't create bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings");
}