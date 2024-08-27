//@ check-pass

#![feature(dyn_star)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn _foo() -> dyn* Unpin {
    4usize
}

fn main() {}

