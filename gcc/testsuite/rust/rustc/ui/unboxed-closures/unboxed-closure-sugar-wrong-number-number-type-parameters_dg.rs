#![feature(unboxed_closures)]

trait Zero { fn dummy(&self); }

fn foo1(_: &dyn Zero()) {
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
}

fn foo2(_: &dyn Zero<usize>) {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

fn foo3(_: &dyn Zero <   usize   >) {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

fn foo4(_: &dyn Zero(usize)) {
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
}

fn foo5(_: &dyn Zero (   usize   )) {
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
}

fn main() { }

