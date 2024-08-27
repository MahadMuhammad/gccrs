#![feature(unboxed_closures)]

fn a<F: Fn<usize>>(f: F) {}
// { dg-error ".E0059." "" { target *-*-* } .-1 }

fn main() {
    a(|_: usize| {}); // { dg-error ".E0308." "" { target *-*-* } }
}

