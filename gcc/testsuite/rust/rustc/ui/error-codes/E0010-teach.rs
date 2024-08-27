//@ compile-flags: -Z teach

#![allow(warnings)]

const CON: Vec<i32> = vec![1, 2, 3]; // { dg-error ".E0015." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
fn main() {}

