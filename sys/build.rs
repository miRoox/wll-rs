extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

fn main() -> Result<(), Box<dyn Error>> {
    let include_path = PathBuf::from("include");
    let wrapper_file = include_path.join("wrapper.h");
    println!("cargo:rustc-link-search=native={}", find_wolfram_library_path()?.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib={}", wolfram_library_name());
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
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum WLError {
    NotFound,
    NotExist,
}

impl Display for WLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            WLError::NotFound => write!(f, "wolfram library not found"),
            WLError::NotExist => write!(f, "wolfram library does not exist"),
        }
    }
}

impl Error for WLError { }

fn find_wolfram_library_path() -> Result<PathBuf, WLError> {
    if let Some(path) = env::var_os("WOLFRAM_LIB") {
        let path = PathBuf::from(path);
        let libpath = path.join(wolfram_library_name());
        if libpath.as_path().exists() {
            Ok(path)
        }
        else {
            Err(WLError::NotExist)
        }
    } else {
        Err(WLError::NotFound)
    }
}

fn wolfram_library_name() -> String {
    "WolframRTL".to_string()
}
