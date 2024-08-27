//@ aux-build:offset-of-staged-api.rs

use std::mem::offset_of;

extern crate offset_of_staged_api;

use offset_of_staged_api::*;

fn main() {
    offset_of!(
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        Unstable, // { dg-error ".E0658." "" { target *-*-* } }
        unstable
    );
    offset_of!(Stable, stable);
    offset_of!(StableWithUnstableField, unstable); // { dg-error ".E0658." "" { target *-*-* } }
    offset_of!(StableWithUnstableFieldType, stable);
    offset_of!(StableWithUnstableFieldType, stable.unstable); // { dg-error ".E0658." "" { target *-*-* } }
    offset_of!(
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        UnstableWithStableFieldType, // { dg-error ".E0658." "" { target *-*-* } }
        unstable
    );
    offset_of!(
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        UnstableWithStableFieldType, // { dg-error ".E0658." "" { target *-*-* } }
        unstable.stable
    );
}

