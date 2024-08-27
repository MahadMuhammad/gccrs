//@ aux-build:non-exhaustive.rs

extern crate non_exhaustive;

use non_exhaustive::NonExhaustiveEnum;

fn main() {
    match Some(NonExhaustiveEnum::A) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-note ".E0004." "" { target *-*-* } .-2 }
// { dg-note ".E0004." "" { target *-*-* } .-3 }
// { dg-note ".E0004." "" { target *-*-* } .-4 }
// { dg-note ".E0004." "" { target *-*-* } .-5 }
        Some(NonExhaustiveEnum::A) => {}
        Some(NonExhaustiveEnum::B) => {}
        None => {}
    }
}

