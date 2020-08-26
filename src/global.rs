//! Some tools to access the global status.

use crate::errors::Error;
use std::cell::RefCell;

thread_local!(
    static CURRENT_LIB_DATA: RefCell<Option<sys::WolframLibraryData>> = RefCell::new(None);
);

/// initialize global `WolframLibraryData`.
#[inline]
pub fn initialize_lib_data(lib_data: Option<sys::WolframLibraryData>) -> Result<(), Error> {
    CURRENT_LIB_DATA.with(|data| data.replace(lib_data));
    Ok(())
}

/// get current `WolframLibraryData`.
#[inline]
pub fn get_lib_data() -> Option<sys::WolframLibraryData> {
    CURRENT_LIB_DATA.with(|data| *data.borrow())
}

/// RAII wrapper to set current `WolframLibraryData` locally.
pub struct LibDataLocalizer {
    old: Option<sys::WolframLibraryData>,
}

impl LibDataLocalizer {
    /// set current `WolframLibraryData` locally.
    #[inline]
    pub fn new(new: Option<sys::WolframLibraryData>) -> Self {
        LibDataLocalizer {
            old: CURRENT_LIB_DATA.with(|data| data.replace(new)),
        }
    }
}

impl Drop for LibDataLocalizer {
    #[inline]
    fn drop(&mut self) {
        CURRENT_LIB_DATA.with(|data| data.replace(self.old));
    }
}
