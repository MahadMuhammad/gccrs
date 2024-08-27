//@ revisions: normal over_aligned

#![feature(dyn_star)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::fmt::Debug;

#[cfg_attr(over_aligned,      repr(C, align(1024)))]
#[cfg_attr(not(over_aligned), repr(C))]
#[derive(Debug)]
struct AlignedUsize(usize);

fn main() {
    let x = AlignedUsize(12) as dyn* Debug;
// { dg-error "" "" { target *-*-* } .-1 }
}

