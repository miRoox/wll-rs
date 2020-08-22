//! # wll
//!
//! Wolfram [LibraryLink] interface for Rust.
//!
//! [LibraryLink]: http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html

#[cfg(not(feature = "no-macros"))]
#[no_link]
pub extern crate wll_macros;
extern crate wll_sys;

pub use complex::Complex;
pub use errors::{Error, ErrorKind};
#[cfg(not(feature = "no-macros"))]
pub use wll_macros::{export, setup, teardown};

pub mod adaptor;
pub mod complex;
pub mod errors;
pub mod global;

/// A specialized `std::result::Result` type for wll functions.
pub type Result<T> = std::result::Result<T, errors::Error>;
