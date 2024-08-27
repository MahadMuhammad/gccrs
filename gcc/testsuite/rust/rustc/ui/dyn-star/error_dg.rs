#![feature(dyn_star)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait Foo {}

fn make_dyn_star() {
    let i = 42;
    let dyn_i: dyn* Foo = i; // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

