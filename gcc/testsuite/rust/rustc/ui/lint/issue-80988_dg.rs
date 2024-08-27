// Regression test for #80988
//
//@ check-pass

#![forbid(warnings)]

#[deny(warnings)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
fn main() {}

