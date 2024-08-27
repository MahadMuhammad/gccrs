//@ aux-build:derive-b.rs

#[macro_use]
extern crate derive_b;

#[B] // { dg-error ".E0659." "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
#[C] // { dg-error "" "" { target *-*-* } }
#[B(D)] // { dg-error ".E0659." "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
#[B(E = "foo")] // { dg-error ".E0659." "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
#[B(arbitrary tokens)] // { dg-error ".E0659." "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
#[derive(B)]
struct B;

fn main() {}

