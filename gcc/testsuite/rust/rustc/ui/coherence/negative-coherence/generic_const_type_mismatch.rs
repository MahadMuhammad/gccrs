//! This test used to ICE (#119381), because relating the `u8` and `i8` generic
//! const with the array length of the `Self` type was succeeding under the
//! assumption that an error had already been reported.

#![feature(with_negative_coherence)]
trait Trait {}
impl<const N: u8> Trait for [(); N] {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
impl<const N: i8> Trait for [(); N] {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

fn main() {}

