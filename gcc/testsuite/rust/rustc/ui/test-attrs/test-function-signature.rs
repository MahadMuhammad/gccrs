//@ compile-flags: --test

#[test]
fn foo() -> Result<(), ()> {
    Ok(())
}

#[test]
fn bar() -> i32 { // { dg-error ".E0277." "" { target *-*-* } }
    0
}

#[test]
fn baz(val: i32) {} // { dg-error "" "" { target *-*-* } }

#[test]
fn lifetime_generic<'a>() -> Result<(), &'a str> {
    Err("coerce me to any lifetime")
}

#[test]
fn type_generic<T>() {} // { dg-error "" "" { target *-*-* } }

#[test]
fn const_generic<const N: usize>() {} // { dg-error "" "" { target *-*-* } }

// Regression test for <https://github.com/rust-lang/rust/issues/112360>. This used to ICE.
fn nested() {
    #[test]
    fn foo(arg: ()) {} // { dg-error "" "" { target *-*-* } }
}

