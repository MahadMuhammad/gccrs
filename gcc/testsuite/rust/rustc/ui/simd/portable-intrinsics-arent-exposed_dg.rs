// May not matter, since people can use them with a nightly feature.
// However this tests to guarantee they don't leak out via portable_simd,
// and thus don't accidentally get stabilized.
use core::simd::intrinsics; // { dg-error ".E0433." "" { target *-*-* } }
use std::simd::intrinsics; // { dg-error ".E0432." "" { target *-*-* } }

fn main() {
    ()
}

