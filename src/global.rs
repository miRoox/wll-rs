//! Some tools to access the global status.

use crate::errors::Error;
use std::mem;

// should be thread local?
pub(crate) static mut CURRENT_LIB_DATA: Option<sys::WolframLibraryData> = None;

/// initialize global `WolframLibraryData`.
#[inline]
pub fn initialize_lib_data(lib_data: Option<sys::WolframLibraryData>) -> Result<(), Error> {
    unsafe {
        CURRENT_LIB_DATA = lib_data;
    }
    Ok(())
}

/// get current `WolframLibraryData`.
#[inline]
pub fn get_lib_data() -> &'static Option<sys::WolframLibraryData> {
    unsafe { &CURRENT_LIB_DATA }
}

/// RAII wrapper to set current `WolframLibraryData` locally.
pub struct LibDataLocalizer {
    old: Option<sys::WolframLibraryData>,
}

impl LibDataLocalizer {
    /// set current `WolframLibraryData` locally.
    #[inline]
    pub fn new(new: Option<sys::WolframLibraryData>) -> Self {
        let mut old = new;
        unsafe {
            mem::swap(&mut old, &mut CURRENT_LIB_DATA);
        }
        LibDataLocalizer { old }
    }
}

impl Drop for LibDataLocalizer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            mem::swap(&mut self.old, &mut CURRENT_LIB_DATA);
        }
    }
}
