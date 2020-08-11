use wll_macros::{wll_setup, wll_teardown};

#[wll_setup]
fn setup() -> ::wll::Result<()> {
    Err(wll::errors::Error::from(
        wll::errors::ErrorKind::FunctionError,
    ))
}

#[wll_teardown]
fn teardown() {}

fn main() {}
