//! Some tools to access the global status.
//! Typically doesn’t need to be used directly.

use crate::{Error, ErrorKind, Result};
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};

static CURRENT_LIB_DATA: AtomicPtr<sys::st_WolframLibraryData> = AtomicPtr::new(ptr::null_mut());

/// Initialize global `WolframLibraryData`.
/// Typically doesn’t need to be used directly.
#[inline]
pub fn initialize_lib_data(lib_data: sys::WolframLibraryData) -> Result<()> {
    if lib_data.is_null() {
        return Err(Error::from(ErrorKind::FunctionError));
    }
    CURRENT_LIB_DATA.store(lib_data, Ordering::Relaxed);
    Ok(())
}

/// Work with current `WolframLibraryData`.
/// Typically doesn’t need to be used directly.
#[inline]
pub fn with_lib_data<F, R>(f: F) -> Result<R>
where
    F: FnOnce(NonNull<sys::st_WolframLibraryData>) -> Result<R>,
{
    if let Some(data) = NonNull::new(CURRENT_LIB_DATA.load(Ordering::Relaxed)) {
        f(data)
    } else {
        Err(Error::from(ErrorKind::FunctionError))
    }
}

/// RAII wrapper to set current `WolframLibraryData` locally.
/// Typically doesn’t need to be used directly.
pub struct LibDataLocalizer {
    old: sys::WolframLibraryData,
}

impl LibDataLocalizer {
    /// set current `WolframLibraryData` locally.
    #[inline]
    pub fn new(new: sys::WolframLibraryData) -> Self {
        LibDataLocalizer {
            old: CURRENT_LIB_DATA.swap(new, Ordering::Release),
        }
    }
}

impl Drop for LibDataLocalizer {
    #[inline]
    fn drop(&mut self) {
        CURRENT_LIB_DATA.swap(self.old, Ordering::Release);
    }
}
