//@ run-rustfix
#![allow(unused_variables, dead_code)]
fn f(x:isize) {
    static child: isize = x + 1;
// { dg-error ".E0435." "" { target *-*-* } .-1 }
}

fn main() {}

