//@ check-fail
#![feature(generic_const_exprs, transmutability)]
#![allow(incomplete_features)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_transmutable<
        Src,
        Dst,
        const ASSUME_ALIGNMENT: bool,
        const ASSUME_LIFETIMES: bool,
        const ASSUME_SAFETY: bool,
        const ASSUME_VALIDITY: bool,
    >()
    where
        Dst: BikeshedIntrinsicFrom<
            Src,
            { from_options(ASSUME_ALIGNMENT, ASSUME_LIFETIMES, ASSUME_SAFETY, ASSUME_VALIDITY) }
        >,
    {}

    // This should not cause an ICE
    const fn from_options(
        , // { dg-error "" "" { target *-*-* } }
        , // { dg-error "" "" { target *-*-* } }
        , // { dg-error "" "" { target *-*-* } }
        , // { dg-error "" "" { target *-*-* } }
    ) -> Assume {} // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {
    #[repr(C)] struct Src;
    #[repr(C)] struct Dst;

    assert::is_transmutable::<Src, Dst, false, false, { true }, false>();
}

