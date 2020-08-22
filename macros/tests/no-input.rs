use std::mem;
use wll::Result;
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(size_bytes)]
fn size() -> Result<usize> {
    Ok(mem::size_of::<usize>())
}

fn main() {}
