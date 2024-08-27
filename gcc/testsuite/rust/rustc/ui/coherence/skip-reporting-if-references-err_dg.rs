// Regression test for #121006.
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

trait ToUnit<'a> {
    type Unit;
}

impl<T> ToUnit for T {}
// { dg-error "" "" { target *-*-* } .-1 }

trait Overlap {}
impl<U> Overlap for fn(U) {}
impl Overlap for for<'a> fn(<() as ToUnit<'a>>::Unit) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}

