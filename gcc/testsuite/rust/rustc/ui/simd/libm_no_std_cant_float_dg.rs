#![crate_type = "rlib"]
#![no_std]
#![feature(portable_simd)]
use core::simd::f32x4;
use core::simd::num::SimdFloat;

// For SIMD float ops, the LLIR version which is used to implement the portable
// forms of them may become calls to math.h AKA libm. So, we can't guarantee
// we can compile them for #![no_std] crates.
// Someday we may solve this.
// Until then, this test at least guarantees these functions require std.
fn guarantee_no_std_nolibm_calls() -> f32x4 {
    let x = f32x4::from_array([0.1, 0.5, 0.6, -1.5]);
    let x2 = x + x;
    let _xc = x.ceil(); // { dg-error ".E0599." "" { target *-*-* } }
    let _xf = x.floor(); // { dg-error ".E0599." "" { target *-*-* } }
    let _xr = x.round(); // { dg-error ".E0599." "" { target *-*-* } }
    let _xt = x.trunc(); // { dg-error ".E0599." "" { target *-*-* } }
    let _xfma = x.mul_add(x, x); // { dg-error ".E0599." "" { target *-*-* } }
    let _xsqrt = x.sqrt(); // { dg-error ".E0599." "" { target *-*-* } }
    x2.abs() * x2
}

