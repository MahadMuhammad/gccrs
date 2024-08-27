// ICE Inconsistent rustc_transmute::is_transmutable(...) result, got Yes
// issue: rust-lang/rust#110969
#![feature(adt_const_params, generic_const_exprs, transmutability)]
#![allow(incomplete_features, unstable_features)]

mod assert {
    use std::mem::BikeshedIntrinsicFrom;

    pub fn is_transmutable<Src, Dst, Context, const ASSUME: std::mem::Assume>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, ASSUME>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    {
    }
}

fn via_associated_const() {
    struct Context;
    #[repr(C)]
    struct Src;
    #[repr(C)]
    struct Dst;

    trait Trait {
        const FALSE: bool = assert::is_transmutable::<Src, Dst, Context, {}>();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

pub fn main() {}

