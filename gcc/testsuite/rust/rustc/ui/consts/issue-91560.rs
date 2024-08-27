// Regression test for issue #91560.

//@ run-rustfix

#![allow(unused,non_upper_case_globals)]

fn foo() {
    let mut length: usize = 2;
// { help "" "" { target *-*-* } .-1 }
    let arr = [0; length];
// { dg-error ".E0435." "" { target *-*-* } .-1 }
}

fn bar() {
    let   length: usize = 2;
// { help "" "" { target *-*-* } .-1 }
    let arr = [0; length];
// { dg-error ".E0435." "" { target *-*-* } .-1 }
}

fn main() {}

