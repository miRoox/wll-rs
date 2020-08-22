use wll_macros::{export, setup, teardown};

#[setup]
fn setup() {}

#[teardown]
fn teardown() {}

type Result = ();
type Option = ();
#[allow(non_camel_case_types)]
type mint = ();
type MArgument = ();
type MType = ();
type MArgumentGetter = ();

#[export(factorial)]
fn fac(n: usize) -> wll::Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}

fn main() {}
