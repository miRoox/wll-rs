extern crate bindgen;

use std::env;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
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
        check_library_path(PathBuf::from(path))
    } else {
        for base in search_wolfram_installations() {
            for rel in relative_library_path() {
                let path = check_library_path(base.join(rel));
                if path.is_ok() {
                    return path;
                }
            }
        }
        Err(WLError::NotFound)
    }
}

fn check_library_path(path: PathBuf) -> Result<PathBuf, WLError> {
    let libpath = path.join(wolfram_library_name());
    if libpath.as_path().exists() {
        Ok(path)
    } else {
        Err(WLError::NotExist)
    }
}

fn search_wolfram_installations() -> Vec<PathBuf> {
    if let Some(paths) = env::var_os("PATH") {
        env::split_paths(&paths)
            .chain(default_wolfram_installations())
            .filter(|p| p.as_path().exists() && p.join(wolfram_kernel_name()).exists())
            .collect()
    } else {
        Vec::new()
    }
}

fn relative_library_path() -> Vec<PathBuf> {
    let path = vec![
        "SystemFiles/Libraries",
        "Contents/SystemFiles/Libraries",
        "Contents/Resources/Wolfram Player.app/Contents/SystemFiles/Libraries",
    ];
    path.iter()
        .map(|s| PathBuf::from(*s).join(system_id()))
        .collect()
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
        .filter_map(|dir| dir.ok().map(|dir| dir.path()))
        .collect()
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn system_id() -> &'static str {
    "Windows-x86-64"
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn system_id() -> &'static str {
    "MacOSX-x86-64"
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn system_id() -> &'static str {
    "Linux-x86-64"
}

fn wolfram_library_name() -> &'static str {
    "WolframRTL"
}
