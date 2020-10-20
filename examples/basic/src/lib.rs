use wll::{ErrorKind, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export]
fn add_two(a: isize, b: isize) -> Result<isize> {
    a.checked_add(b)
        .ok_or_else(|| ErrorKind::NumericalError.into())
}

#[wll::export(wfactorial)]
fn wfac(n: usize) -> Result<usize> {
    Ok(if n == 0 {
        1
    } else {
        wfac(n - 1)?.wrapping_mul(n)
    })
}
