// An unknown destination type should be gracefully handled.

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::BikeshedIntrinsicFrom;

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src>
    {}
}

fn should_gracefully_handle_unknown_src_field() {
    #[repr(C)] struct Src(Missing); // { dg-error ".E0412." "" { target *-*-* } }
    #[repr(C)] struct Dst();
    assert::is_transmutable::<Src, Dst>(); // { dg-error ".E0277." "" { target *-*-* } }
}

fn should_gracefully_handle_unknown_src_ref_field() {
    #[repr(C)] struct Src(&'static Missing); // { dg-error ".E0412." "" { target *-*-* } }
    #[repr(C)] struct Dst(&'static Dst);
    assert::is_transmutable::<Src, Dst>(); // { dg-error ".E0277." "" { target *-*-* } }
}

