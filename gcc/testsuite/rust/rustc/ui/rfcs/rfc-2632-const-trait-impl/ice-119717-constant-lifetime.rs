#![allow(incomplete_features)]
#![feature(const_trait_impl, effects, try_trait_v2)]

use std::ops::FromResidual;

impl<T> const FromResidual for T {
// { dg-error ".E0210." "" { target *-*-* } .-1 }
// { dg-error ".E0210." "" { target *-*-* } .-2 }
    fn from_residual(t: T) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        t
    }
}

fn main() {}

