// An unknown destination type should be gracefully handled.

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(incomplete_features)]

mod assert {
    use std::mem::TransmuteFrom;

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src>
    {}
}

fn should_gracefully_handle_unknown_dst_field() {
    #[repr(C)] struct Src;
    #[repr(C)] struct Dst(Missing); // { dg-error ".E0412." "" { target *-*-* } }
    assert::is_transmutable::<Src, Dst>(); // { dg-error ".E0277." "" { target *-*-* } }
}

fn should_gracefully_handle_unknown_dst_ref_field() {
    #[repr(C)] struct Src(&'static Src);
    #[repr(C)] struct Dst(&'static Missing); // { dg-error ".E0412." "" { target *-*-* } }
    assert::is_transmutable::<Src, Dst>(); // { dg-error ".E0277." "" { target *-*-* } }
}

