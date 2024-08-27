#![feature(transmutability)]

use std::mem::{Assume, BikeshedIntrinsicFrom};

#[repr(C)]
struct W<'a>(&'a ());

fn test<'a>()
where
    W<'a>: BikeshedIntrinsicFrom<
            (),
            { Assume { alignment: true, lifetimes: true, safety: true, validity: true } },
        >,
{
}

fn main() {
    test();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

