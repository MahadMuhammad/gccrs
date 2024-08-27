//@ compile-flags: -Z unstable-options

#![feature(rustc_attrs)]

#[rustc_lint_diagnostics]
// { dg-error "" "" { target *-*-* } .-1 }
struct Foo;

impl Foo {
    #[rustc_lint_diagnostics(a)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn bar() {}
}

fn main() {}

