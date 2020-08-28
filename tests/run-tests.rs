#[test]
#[cfg_attr(not(feature = "macros"), ignore)]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/empty-setup.rs");
    t.pass("tests/ui/setup-return.rs");
    t.pass("tests/ui/add-two.rs");
    t.pass("tests/ui/factorial.rs");
    t.compile_fail("tests/ui/conflict-name.rs");
    t.pass("tests/ui/complex-conj.rs");
    t.compile_fail("tests/ui/complex-int-input.rs");
    t.pass("tests/ui/complex-int-output.rs");
    t.pass("tests/ui/no-input.rs");
    t.pass("tests/ui/void-return.rs");
    t.pass("tests/ui/do-nothing.rs");
    t.compile_fail("tests/ui/missing-result.rs");
    t.compile_fail("tests/ui/missing-return.rs");
    t.pass("tests/ui/redefine-prelude.rs");
}
