use wll::{Complex, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(conj)]
fn cconj(z: Complex<i64>) -> Result<Complex<i64>> {
    Ok(z.conj())
}

fn main() {}
