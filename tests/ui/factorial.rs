use wll::Result;

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(factorial)]
fn fac(n: usize) -> Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}

fn main() {}
