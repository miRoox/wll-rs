extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let include_path = PathBuf::from("include");
    let wrapper_file = include_path.join("wrapper.h");
    if cfg!(feature = "auto-link") {
        link_libraries();
    }
    println!("cargo:rerun-if-changed={}", include_path.to_str().unwrap());
    let bindings = bindgen::Builder::default()
        .header(wrapper_file.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn link_libraries() {
    if !env::var("DOCS_RS").is_ok() {
        find_library_paths();
        println!("cargo:rustc-link-lib=dylib={}", wolfram_library_name());
    }
}

fn find_library_paths() {
    let start = "-- RUNTIME_LIBRARY_DIRS=";
    let output = Command::new("cmake")
        .current_dir("wlocate")
        .arg(".")
        .output()
        .expect("Failed to execute cmake");
    let msg = String::from_utf8(output.stdout).expect("Invalid character in output of cmake");
    msg.lines()
        .find(|s| s.starts_with(start))
        .expect("Do not find Wolfram Runtime Library")
        .trim_start_matches(start)
        .split_terminator(';')
        .for_each(|p| println!("cargo:rustc-link-search=native={}", p));
}

fn wolfram_library_name() -> &'static str {
    "WolframRTL"
}
