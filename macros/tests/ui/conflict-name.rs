use wll::{Error, ErrorKind, Result};
use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

#[export(add_two)]
fn add_two(a: isize, b: isize) -> Result<isize> {
    a.checked_add(b)
        .ok_or_else(|| Error::from(ErrorKind::NumericalError))
}

fn main() {}
