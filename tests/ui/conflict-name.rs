use wll::{Error, ErrorKind, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(add_two)]
fn add_two(a: isize, b: isize) -> Result<isize> {
    a.checked_add(b)
        .ok_or_else(|| Error::from(ErrorKind::NumericalError))
}

fn main() {}
