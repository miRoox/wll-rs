#![allow(dead_code)]

#[wll::setup]
fn setup() {}

#[wll::teardown]
fn teardown() {}

type Result = ();
type Option = ();
#[allow(non_camel_case_types)]
type mint = ();
type MArgument = ();
type MType = ();
type MArgumentGetter = ();

#[wll::export(factorial)]
fn fac(n: usize) -> wll::Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}

fn main() {}
