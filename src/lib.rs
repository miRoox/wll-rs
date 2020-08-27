//! Wolfram [LibraryLink] interface for Rust.
//!
//! # Examples
//!
//! Examples can found in the examples directory of the source code, or on [GitHub](https://github.com/miRoox/wll-rs/tree/master/examples).
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
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
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
#[inline]
pub fn message(msg: &'static str) -> Result<()> {
    global::with_lib_data(|data| unsafe {
        let func = (*data.as_ptr())
            .Message
            .unwrap_or_else(|| std::hint::unreachable_unchecked());
        func(msg.as_ptr() as *const ::std::os::raw::c_char);
        Ok(())
    })
}

/// Checks if the Wolfram Language is in the process of an abort.
///
/// **see also**: [AbortQ](http://reference.wolfram.com/language/LibraryLink/ref/callback/AbortQ.html)
#[inline]
pub fn is_abort() -> Result<bool> {
    global::with_lib_data(|data| unsafe {
        Ok((*data.as_ptr())
            .AbortQ
            .unwrap_or_else(|| std::hint::unreachable_unchecked())()
            != 0)
    })
}
