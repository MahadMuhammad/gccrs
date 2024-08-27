#![feature(portable_simd)]
#![feature(generic_arg_infer)]
use std::simd::Mask;

fn main() {
    let y = Mask::<_, _>::splat(false);
// { dg-error ".E0284." "" { target *-*-* } .-1 }
// { dg-error ".E0284." "" { target *-*-* } .-2 }
}

