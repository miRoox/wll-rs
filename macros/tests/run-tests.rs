#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/empty-setup.rs");
    t.pass("tests/setup-return.rs");
    t.pass("tests/add-two.rs");
    t.pass("tests/factorial.rs");
    t.compile_fail("tests/conflict-name.rs");
    t.pass("tests/complex-conj.rs");
    t.compile_fail("tests/complex-int-input.rs");
    t.pass("tests/complex-int-output.rs");
    t.pass("tests/no-input.rs");
    t.pass("tests/void-return.rs");
    t.pass("tests/do-nothing.rs");
    t.compile_fail("tests/missing-result.rs");
    t.compile_fail("tests/missing-return.rs");
    t.pass("tests/redefine-prelude.rs");
}
