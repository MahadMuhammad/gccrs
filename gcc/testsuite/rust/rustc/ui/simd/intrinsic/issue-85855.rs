// Check that appropriate errors are reported if an intrinsic is defined
// with the wrong number of generic lifetime/type/const parameters, and
// that no ICE occurs in these cases.

#![feature(intrinsics)]
#![crate_type="lib"]

extern "rust-intrinsic" {
    fn simd_saturating_add<'a, T: 'a>(x: T, y: T);
// { dg-error ".E0094." "" { target *-*-* } .-1 }

    fn simd_add<'a, T>(x: T, y: T) -> T;

    fn simd_sub<T, U>(x: T, y: U);
// { dg-error ".E0094." "" { target *-*-* } .-1 }

    fn simd_mul<T, const N: usize>(x: T, y: T);
// { dg-error ".E0094." "" { target *-*-* } .-1 }
}

