//@ aux-build:enums.rs

extern crate enums;

use enums::FieldLessWithNonExhaustiveVariant;

fn main() {
    let e = FieldLessWithNonExhaustiveVariant::default();
    let d = e as u8; // { dg-error ".E0606." "" { target *-*-* } }
    assert_eq!(d, 0);
}

