//@build-fail
//! Make sure that monomorphization-time const errors from `static_assert` take priority over the
//! error from simd_extract. Basically this checks that if a const fails to evaluate in some
//! function, we don't bother codegen'ing the function.
#![feature(generic_arg_infer)]
#![feature(core_intrinsics)]
#![feature(repr_simd)]

use std::intrinsics::simd::*;

#[repr(simd)]
#[allow(non_camel_case_types)]
struct int8x4_t(u8,u8,u8,u8);

fn get_elem<const LANE: u32>(a: int8x4_t) -> u8 {
    const { assert!(LANE < 4); } // the error should be here...
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
    unsafe { simd_extract(a, LANE) } // ...not here
}

fn main() {
    get_elem::<4>(int8x4_t(0,0,0,0));
}

