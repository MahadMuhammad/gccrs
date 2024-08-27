// Test the enum_intrinsics_non_enums lint.

#![feature(variant_count)]

use std::mem::{discriminant, variant_count};

enum SomeEnum {
    A,
    B,
}

struct SomeStruct;

fn generic_discriminant<T>(v: &T) {
    discriminant::<T>(v);
}

fn generic_variant_count<T>() -> usize {
    variant_count::<T>()
}

fn test_discriminant() {
    discriminant(&SomeEnum::A);
    generic_discriminant(&SomeEnum::B);

    discriminant(&());
// { dg-error "" "" { target *-*-* } .-1 }

    discriminant(&&SomeEnum::B);
// { dg-error "" "" { target *-*-* } .-1 }

    discriminant(&SomeStruct);
// { dg-error "" "" { target *-*-* } .-1 }

    discriminant(&123u32);
// { dg-error "" "" { target *-*-* } .-1 }

    discriminant(&&123i8);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_variant_count() {
    variant_count::<SomeEnum>();
    generic_variant_count::<SomeEnum>();

    variant_count::<&str>();
// { dg-error "" "" { target *-*-* } .-1 }

    variant_count::<*const u8>();
// { dg-error "" "" { target *-*-* } .-1 }

    variant_count::<()>();
// { dg-error "" "" { target *-*-* } .-1 }

    variant_count::<&SomeEnum>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    test_discriminant();
    test_variant_count();

    // The lint ignores cases where the type is generic, so these should be
    // allowed even though their return values are unspecified
    generic_variant_count::<SomeStruct>();
    generic_discriminant::<SomeStruct>(&SomeStruct);
}

