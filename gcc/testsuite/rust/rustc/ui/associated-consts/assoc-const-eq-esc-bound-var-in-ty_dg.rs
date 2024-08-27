// Detect and reject escaping late-bound generic params in
// the type of assoc consts used in an equality bound.
#![feature(associated_const_equality)]

trait Trait<'a> {
    const K: &'a ();
}

fn take(_: impl for<'r> Trait<'r, K = { &() }>) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

fn main() {}

