#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! # wll-sys
//!
//! `wll-sys` is a low-level bindings for Wolfram Library.
//!
//! **see also**: [Wolfram LibraryLink User Guide](http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html).

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
