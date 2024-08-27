//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(do_not_recommend)]

trait Foo {}

#[diagnostic::do_not_recommend]
impl<T> Foo for T where T: Send {}

fn needs_foo<T: Foo>() {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

fn main() {
    needs_foo::<*mut ()>();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}

