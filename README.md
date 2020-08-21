# wll-rs

Wolfram [LibraryLink](http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html) interface for Rust

Inspired by [wll-interface](https://github.com/njpipeorgan/wll-interface).

Purpose:

```rust
// lib.rs
use wll::Result;
use wll::error::{Error, ErrorKind};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

// export function named `wll_add_two`
#[wll_export]
fn add_two(a: isize, b: isize)->Result<isize> {
    a.checked_add(b)
     .ok_or_else(|| Error::from(ErrorKind::NumericalError))
}

#[wll_export(factorial)]
fn fac(n: isize)->Result<isize> {
    Ok(n*fac(n-1)?)
}
```
