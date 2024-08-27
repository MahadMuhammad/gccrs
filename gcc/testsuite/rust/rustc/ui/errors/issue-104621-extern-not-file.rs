//@ compile-flags: --extern foo=.

extern crate foo; // { dg-error "" "" { target *-*-* } }
fn main() {}

