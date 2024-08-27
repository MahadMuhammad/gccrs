#![crate_type = "lib"]
#![feature(transmutability)]
mod assert {
    use std::mem::TransmuteFrom;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src>,
    {
    }
}

fn main() {
    pub union Uninit {
        a: [u8; usize::MAX],
    }

    #[repr(C)]
    struct ExplicitlyPadded(Uninit);

    assert::is_maybe_transmutable::<(), ExplicitlyPadded>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    assert::is_maybe_transmutable::<ExplicitlyPadded, ()>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

