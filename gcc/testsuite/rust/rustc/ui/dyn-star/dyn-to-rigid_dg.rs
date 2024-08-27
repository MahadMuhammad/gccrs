#![feature(dyn_star)]
#![allow(incomplete_features)]

trait Tr {}

fn f(x: dyn* Tr) -> usize {
    x as usize
// { dg-error ".E0606." "" { target *-*-* } .-1 }
}

fn main() {}

