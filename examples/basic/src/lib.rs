use wll::{Error, ErrorKind, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export]
fn add_two(a: isize, b: isize) -> Result<isize> {
    a.checked_add(b)
        .ok_or_else(|| Error::from(ErrorKind::NumericalError))
}

#[wll::export(factorial)]
fn fac(n: usize) -> Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}
