use crate::errors::{Error, ErrorKind};
use std::mem;

// should be thread local?
pub(crate) static mut CURRENT_LIB_DATA: Option<wll_sys::WolframLibraryData> = None;

/// initialize global `WolframLibraryData`.
#[inline]
pub unsafe fn initialize_lib_data(
    lib_data: Option<wll_sys::WolframLibraryData>,
) -> Result<(), Error> {
    if lib_data.is_none() || CURRENT_LIB_DATA.is_some() {
        return Err(Error::from(ErrorKind::FunctionError));
    }
    CURRENT_LIB_DATA = lib_data;
    Ok(())
}

/// get current `WolframLibraryData`.
#[inline]
pub unsafe fn get_lib_data() -> &'static Option<wll_sys::WolframLibraryData> {
    &CURRENT_LIB_DATA
}

/// RAII wrapper to set current `WolframLibraryData` locally.
pub struct LibDataLocalizer {
    old: Option<wll_sys::WolframLibraryData>,
}

impl LibDataLocalizer {
    /// set current `WolframLibraryData` locally.
    #[inline]
    pub fn new(new: Option<wll_sys::WolframLibraryData>) -> Self {
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
