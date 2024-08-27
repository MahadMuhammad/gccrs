//@ check-fail

//! Reject lifetime extensions.

#![feature(transmutability, core_intrinsics)]

use std::mem::{Assume, BikeshedIntrinsicFrom};

unsafe fn transmute<Src, Dst>(src: Src) -> Dst
where
    Dst: BikeshedIntrinsicFrom<Src, { Assume::SAFETY }>,
{
    core::intrinsics::transmute_unchecked(src)
}

mod bare {
    use super::*;

    fn extend_bare<'a>(src: &'a u8) -> &'static u8 {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }
}

mod nested {
    use super::*;

    fn extend_nested<'a>(src: &'a &'a u8) -> &'a &'static u8 {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }
}

mod tuple {
    use super::*;

    fn extend_unit<'a>(src: (&'a u8,)) -> (&'static u8,) {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }

    fn extend_pair<'a>(src: (&'a u8, u8)) -> (&'static u8, u8) {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }
}

mod r#struct {
    use super::*;

    struct Struct<'a>(&'a u8);

    fn extend_struct<'a>(src: Struct<'a>) -> Struct<'static> {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }
}

mod r#enum {
    use super::*;

    enum Single<'a> {
        A(&'a u8),
    }

    fn extend_single<'a>(src: Single<'a>) -> Single<'static> {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }

    enum Multi<'a> {
        A(&'a u8),
        B,
        C,
    }

    fn extend_multi<'a>(src: Multi<'a>) -> Multi<'static> {
        unsafe { transmute(src) } // { dg-error "" "" { target *-*-* } }
    }
}

mod hrtb {
    use super::*;

    fn call_extend_hrtb<'a>(src: &'a u8) -> &'static u8 {
        unsafe { extend_hrtb(src) } // { dg-error ".E0521." "" { target *-*-* } }
    }

    unsafe fn extend_hrtb<'a>(src: &'a u8) -> &'static u8
    where
        for<'b> &'b u8: BikeshedIntrinsicFrom<&'a u8>,
    {
        core::intrinsics::transmute_unchecked(src)
    }
}

fn main() {}

