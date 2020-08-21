use std::mem;
use wll::Result;
use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export(size_bytes)]
fn size() -> Result<usize> {
    Ok(mem::size_of::<usize>())
}

fn main() {}
