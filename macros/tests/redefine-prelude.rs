use wll_macros::{wll_export, wll_setup, wll_teardown};

#[wll_setup]
fn setup() {}

#[wll_teardown]
fn teardown() {}

type Result = ();
type Option = ();
#[allow(non_camel_case_types)]
type mint = ();
type MArgument = ();
type MType = ();
type MArgumentGetter = ();

#[wll_export(factorial)]
fn fac(n: usize) -> wll::Result<usize> {
    Ok(if n == 0 { 1 } else { n * fac(n - 1)? })
}

fn main() {}
