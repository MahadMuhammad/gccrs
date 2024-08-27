#![allow(incomplete_features)]
#![feature(explicit_tail_calls)]

const fn f() {
    become dangerous();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

const unsafe fn dangerous() {}

fn main() {}

