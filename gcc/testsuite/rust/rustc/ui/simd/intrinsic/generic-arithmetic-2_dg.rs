//@ build-fail

#![feature(repr_simd, intrinsics)]
#![allow(non_camel_case_types)]
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x4(pub i32, pub i32, pub i32, pub i32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

extern "rust-intrinsic" {
    fn simd_add<T>(x: T, y: T) -> T;
    fn simd_sub<T>(x: T, y: T) -> T;
    fn simd_mul<T>(x: T, y: T) -> T;
    fn simd_div<T>(x: T, y: T) -> T;
    fn simd_rem<T>(x: T, y: T) -> T;
    fn simd_shl<T>(x: T, y: T) -> T;
    fn simd_shr<T>(x: T, y: T) -> T;
    fn simd_and<T>(x: T, y: T) -> T;
    fn simd_or<T>(x: T, y: T) -> T;
    fn simd_xor<T>(x: T, y: T) -> T;

    fn simd_neg<T>(x: T) -> T;
    fn simd_bswap<T>(x: T) -> T;
    fn simd_bitreverse<T>(x: T) -> T;
    fn simd_ctlz<T>(x: T) -> T;
    fn simd_ctpop<T>(x: T) -> T;
    fn simd_cttz<T>(x: T) -> T;
}

fn main() {
    let x = i32x4(0, 0, 0, 0);
    let y = u32x4(0, 0, 0, 0);
    let z = f32x4(0.0, 0.0, 0.0, 0.0);

    unsafe {
        simd_add(x, x);
        simd_add(y, y);
        simd_add(z, z);
        simd_sub(x, x);
        simd_sub(y, y);
        simd_sub(z, z);
        simd_mul(x, x);
        simd_mul(y, y);
        simd_mul(z, z);
        simd_div(x, x);
        simd_div(y, y);
        simd_div(z, z);
        simd_rem(x, x);
        simd_rem(y, y);
        simd_rem(z, z);

        simd_shl(x, x);
        simd_shl(y, y);
        simd_shr(x, x);
        simd_shr(y, y);
        simd_and(x, x);
        simd_and(y, y);
        simd_or(x, x);
        simd_or(y, y);
        simd_xor(x, x);
        simd_xor(y, y);

        simd_neg(x);
        simd_neg(z);
        simd_bswap(x);
        simd_bswap(y);
        simd_bitreverse(x);
        simd_bitreverse(y);
        simd_ctlz(x);
        simd_ctlz(y);
        simd_cttz(x);
        simd_cttz(y);

        simd_add(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_sub(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_mul(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_div(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shl(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shr(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_and(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_or(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_xor(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        simd_neg(0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_bswap(0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_bitreverse(0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_ctlz(0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_cttz(0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        simd_shl(z, z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shr(z, z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_and(z, z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_or(z, z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_xor(z, z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_bswap(z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_bitreverse(z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_ctlz(z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_ctpop(z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_cttz(z);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
    }
}

