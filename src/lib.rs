//! # wll
//!
//! Wolfram [LibraryLink] interface for Rust.
//!
//! [LibraryLink]: http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html

#[cfg(feature = "macros")]
#[no_link]
#[allow(unused_imports)]
#[macro_use]
extern crate wll_macros;
#[doc(hidden)]
pub extern crate wll_sys as sys;

pub use complex::Complex;
pub use errors::{Error, ErrorKind};
#[cfg(feature = "macros")]
pub use wll_macros::*;

pub mod adaptor;
pub mod global;

mod complex;
mod errors;

/// A specialized `std::result::Result` type for wll functions.
pub type Result<T> = std::result::Result<T, Error>;

/// Issues a message from a library function.
///
/// **see also**: [Message](http://reference.wolfram.com/language/LibraryLink/ref/callback/Message.html)
pub fn message(msg: &'static str) -> Result<()> {
    global::with_lib_data(|data| unsafe {
        if let Some(func) = (*data.as_ptr()).Message {
            func(msg.as_ptr() as *const ::std::os::raw::c_char);
            return Ok(());
        }
        Err(Error::from(ErrorKind::FunctionError))
    })
}
