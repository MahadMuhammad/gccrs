#![feature(dyn_star)]
#![allow(incomplete_features)]

use std::fmt::Debug;

fn main() {
    let i = 42 as &dyn* Debug;
// { dg-error ".E0605." "" { target *-*-* } .-1 }
}

