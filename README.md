# wll-rs

[![crates.io](https://img.shields.io/crates/v/wll.svg)](https://crates.io/crates/wll)
[![doc.rs](https://docs.rs/wll/badge.svg)](https://docs.rs/wll)
[![CI](https://github.com/miRoox/wll-rs/workflows/CI/badge.svg)](https://github.com/miRoox/wll-rs/actions?query=workflow%3ACI)

Wolfram [LibraryLink](http://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html) interface for Rust

Inspired by [wll-interface](https://github.com/njpipeorgan/wll-interface).

Purpose:

```rust
// lib.rs
use wll::{ErrorKind, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

// export function named `wll_add_two`
#[wll::export]
fn add_two(a: isize, b: isize)->Result<isize> {
    a.checked_add(b)
     .ok_or_else(|| ErrorKind::NumericalError.into())
}

#[wll::export(factorial)]
fn fac(n: usize) -> Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}
```
