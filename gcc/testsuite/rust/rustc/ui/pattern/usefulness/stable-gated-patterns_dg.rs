//@ aux-build:unstable.rs

extern crate unstable;

use unstable::UnstableEnum;

fn main() {
    match UnstableEnum::Stable {
        UnstableEnum::Stable => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-3 }

    match UnstableEnum::Stable {
        UnstableEnum::Stable => {}
        UnstableEnum::Stable2 => {}
    }
// { dg-error ".E0004." "" { target *-*-* } .-4 }
}

