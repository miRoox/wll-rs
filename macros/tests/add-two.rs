use wll::errors::{Error, ErrorKind};
use wll::Result;
use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

#[wll_export]
fn add_two(a: isize, b: isize) -> Result<isize> {
    a.checked_add(b)
        .ok_or_else(|| Error::from(ErrorKind::NumericalError))
}

fn main() {}
