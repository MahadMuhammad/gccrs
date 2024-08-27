#![feature(transmutability)]

mod assert {
    use std::mem::TransmuteFrom;

    pub fn is_transmutable<Src, const ASSUME_ALIGNMENT: bool>()
    where
        Dst: TransmuteFrom<Src, ASSUME_ALIGNMENT>, // { dg-error ".E0308." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
    {
    }
}

fn via_const() {
    struct Src;

    assert::is_transmutable::<Src, false>();
}

fn main() {}

