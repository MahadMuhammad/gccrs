#![allow(incomplete_features)]
#![feature(const_trait_impl, effects, try_trait_v2, const_try)]
use std::ops::{FromResidual, Try};

struct TryMe;
struct Error;

impl const FromResidual<Error> for TryMe {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }

impl const Try for TryMe {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
    type Output = ();
    type Residual = Error;
}

const fn t() -> TryMe {
    TryMe?;
    TryMe
}

const _: () = {
    t();
};

fn main() {}

