extern crate bindgen;

use std::env;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::iter;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let include_path = PathBuf::from("include");
    let wrapper_file = include_path.join("wrapper.h");
    println!(
        "cargo:rustc-link-search=native={}",
        find_wolfram_library_path()?.to_str().unwrap()
    );
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

impl Error for WLError {}

fn find_wolfram_library_path() -> Result<PathBuf, WLError> {
    if let Some(path) = env::var_os("WOLFRAM_LIB") {
        let path = PathBuf::from(path);
        let libpath = path.join(wolfram_library_name());
        if libpath.as_path().exists() {
            Ok(path)
        } else {
            Err(WLError::NotExist)
        }
    } else {
        Err(WLError::NotFound)
    }
}

fn search_wolfram_installations() -> Vec<PathBuf> {
    if let Some(paths) = env::var_os("PATH") {
        env::split_paths(&paths)
            .join(default_wolfram_installations())
            .filter(|p| p.as_path().exists() && p.join(wolfram_kernel_name()).exists())
            .collect()
    } else {
        Vec::new()
    }
}

#[cfg(target_os = "windows")]
fn wolfram_kernel_name() -> &'static str {
    "math.exe"
}

#[cfg(not(target_os = "windows"))]
fn wolfram_kernel_name() -> &'static str {
    "math"
}

// see http://reference.wolfram.com/language/ref/$InstallationDirectory.html
fn default_wolfram_installations() -> Vec<PathBuf> {
    let base = if cfg!(target_os = "windows") {
        "C:\\Program Files\\Wolfram Research\\Mathematica\\"
    } else if cfg!(target_os = "linux") {
        "/usr/local/Wolfram/Mathematica/"
    } else if cfg!(target_os = "macos") {
        return vec![PathBuf::from("/Applications/Mathematica.app/Contents")];
    } else {
        return Vec::new();
    };
    let base = PathBuf::from(base);
    if !base.is_dir() {
        return Vec::new();
    }
    base.as_path()
        .read_dir()
        .expect("read_dir call failed")
        .filter_map(Result::ok)
        .collect()
}

fn wolfram_library_name() -> String {
    "WolframRTL".to_string()
}
