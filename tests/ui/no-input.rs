use std::mem;
use wll::Result;

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(size_bytes)]
fn size() -> Result<usize> {
    Ok(mem::size_of::<usize>())
}

fn main() {}
