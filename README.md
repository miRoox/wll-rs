# wll-rs

Wolfram [LibraryLink](http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html) interface for Rust

Purpose:

```rust
// lib.rs
use wll::Result;
use wll::error::{Error, ErrorKind};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export]
fn add_two(a: isize, b: isize)->Result<isize> {
    a.checked_add(b)
     .ok_or_else(|| Error::from(ErrorKind::NumericalError))
}
```
