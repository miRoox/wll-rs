use wll_macros::{setup, teardown};

#[setup]
fn setup() -> ::wll::Result<()> {
    Err(wll::errors::Error::from(
        wll::errors::ErrorKind::FunctionError,
    ))
}

#[teardown]
fn teardown() {}

fn main() {}
