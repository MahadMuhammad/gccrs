// Regression test for issue #91227.

//@ run-rustfix

#![allow(unused_macros)]

marco_rules! thing {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    () => {}
}

fn main() {}

