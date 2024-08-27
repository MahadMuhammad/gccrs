//@ check-pass

#![deny(multiple_supertrait_upcastable)]
// { dg-warning "" "" { target *-*-* } .-1 }
#![warn(multiple_supertrait_upcastable)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {}

