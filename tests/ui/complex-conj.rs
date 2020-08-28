use wll::{Complex, Result};

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

#[wll::export(conj)]
fn cconj(z: Complex<f64>) -> Result<Complex<f64>> {
    Ok(z.conj())
}

fn main() {}
