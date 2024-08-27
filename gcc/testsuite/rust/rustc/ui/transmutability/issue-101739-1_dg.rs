#![feature(transmutability)]

mod assert {
    use std::mem::BikeshedIntrinsicFrom;

    pub fn is_transmutable<Src, const ASSUME_ALIGNMENT: bool>()
    where
        Dst: BikeshedIntrinsicFrom<Src, ASSUME_ALIGNMENT>, // { dg-error ".E0412." "" { target *-*-* } }
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    {
    }
}

fn via_const() {
    struct Src;

    assert::is_transmutable::<Src, false>();
}

fn main() {}

