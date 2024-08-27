#![feature(unstable_test_feature)]

//@ aux-build:unstable.rs

extern crate unstable;

use unstable::UnstableStruct;

fn main() {
    let UnstableStruct { stable, stable2, } = UnstableStruct::default();
// { dg-error ".E0027." "" { target *-*-* } .-1 }

    let UnstableStruct { stable, unstable, } = UnstableStruct::default();
// { dg-error ".E0027." "" { target *-*-* } .-1 }

    // OK: stable field is matched
    let UnstableStruct { stable, stable2, unstable } = UnstableStruct::default();
}

