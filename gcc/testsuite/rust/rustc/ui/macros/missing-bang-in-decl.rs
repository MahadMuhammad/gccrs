//@ run-rustfix

#![allow(unused_macros)]

macro_rules foo {
// { dg-error "" "" { target *-*-* } .-1 }
    () => {};
}

macro_rules bar! {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    () => {};
}

fn main() {}

