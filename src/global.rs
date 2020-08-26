//! Some tools to access the global status.

use crate::errors::Error;
use std::cell::RefCell;
use std::ptr;

static CURRENT_LIB_DATA: RefCell<sys::WolframLibraryData> = RefCell::new(ptr::null_mut());

/// initialize global `WolframLibraryData`.
#[inline]
pub fn initialize_lib_data(lib_data: sys::WolframLibraryData) -> Result<(), Error> {
    CURRENT_LIB_DATA.replace(lib_data);
    Ok(())
}

/// get current `WolframLibraryData`.
#[inline]
pub fn get_lib_data() -> sys::WolframLibraryData {
    *CURRENT_LIB_DATA.borrow()
}

/// RAII wrapper to set current `WolframLibraryData` locally.
pub struct LibDataLocalizer {
    old: sys::WolframLibraryData,
}

impl LibDataLocalizer {
    /// set current `WolframLibraryData` locally.
    #[inline]
    pub fn new(new: sys::WolframLibraryData) -> Self {
        LibDataLocalizer {
            old: CURRENT_LIB_DATA.replace(new),
        }
    }
}

impl Drop for LibDataLocalizer {
    #[inline]
    fn drop(&mut self) {
        CURRENT_LIB_DATA.replace(self.old);
    }
}
