#![feature(transmutability)]

use std::mem::{Assume, TransmuteFrom};

#[repr(C)]
struct W<'a>(&'a ());

fn test<'a>()
where
    W<'a>: TransmuteFrom<
            (),
            { Assume { alignment: true, lifetimes: true, safety: true, validity: true } },
        >,
{
}

fn main() {
    test();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

