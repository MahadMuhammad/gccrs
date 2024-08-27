//@ aux-build:hidden.rs

extern crate hidden;

use hidden::HiddenEnum;

enum InCrate {
    A,
    B,
    #[doc(hidden)]
    C,
}

fn main() {
    match HiddenEnum::A {
        HiddenEnum::A => {}
        HiddenEnum::B => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-4 }

    match HiddenEnum::A {
        HiddenEnum::A => {}
        HiddenEnum::C => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-4 }

    match HiddenEnum::A {
        HiddenEnum::A => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-3 }

    match None {
        None => {}
        Some(HiddenEnum::A) => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-4 }

    match InCrate::A {
        InCrate::A => {}
        InCrate::B => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-4 }
}

