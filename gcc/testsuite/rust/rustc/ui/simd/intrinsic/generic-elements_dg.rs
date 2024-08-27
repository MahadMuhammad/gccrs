//@ build-fail

#![feature(repr_simd, intrinsics, rustc_attrs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

#[repr(simd)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct i32x2(i32, i32);
#[repr(simd)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct i32x4(i32, i32, i32, i32);
#[repr(simd)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct i32x8(i32, i32, i32, i32, i32, i32, i32, i32);

#[repr(simd)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct f32x2(f32, f32);
#[repr(simd)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct f32x4(f32, f32, f32, f32);
#[repr(simd)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
struct f32x8(f32, f32, f32, f32, f32, f32, f32, f32);

extern "rust-intrinsic" {
    fn simd_insert<T, E>(x: T, idx: u32, y: E) -> T;
    fn simd_extract<T, E>(x: T, idx: u32) -> E;

    fn simd_shuffle<T, I, U>(x: T, y: T, idx: I) -> U;
    fn simd_shuffle_generic<T, U, const IDX: &'static [u32]>(x: T, y: T) -> U;
}

fn main() {
    let x = i32x4(0, 0, 0, 0);

    unsafe {
        simd_insert(0, 0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_insert(x, 0, 1.0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_extract::<_, f32>(x, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        const IDX2: [u32; 2] = [0; 2];
        simd_shuffle::<i32, _, i32>(0, 0, IDX2);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        const IDX4: [u32; 4] = [0; 4];
        simd_shuffle::<i32, _, i32>(0, 0, IDX4);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        const IDX8: [u32; 8] = [0; 8];
        simd_shuffle::<i32, _, i32>(0, 0, IDX8);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        simd_shuffle::<_, _, f32x2>(x, x, IDX2);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle::<_, _, f32x4>(x, x, IDX4);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle::<_, _, f32x8>(x, x, IDX8);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        simd_shuffle::<_, _, i32x8>(x, x, IDX2);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle::<_, _, i32x8>(x, x, IDX4);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle::<_, _, i32x2>(x, x, IDX8);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        const I2: &[u32] = &[0; 2];
        simd_shuffle_generic::<i32, i32, I2>(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        const I4: &[u32] = &[0; 4];
        simd_shuffle_generic::<i32, i32, I4>(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        const I8: &[u32] = &[0; 8];
        simd_shuffle_generic::<i32, i32, I8>(0, 0);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        simd_shuffle_generic::<_, f32x2, I2>(x, x);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle_generic::<_, f32x4, I4>(x, x);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle_generic::<_, f32x8, I8>(x, x);
// { dg-error ".E0511." "" { target *-*-* } .-1 }

        simd_shuffle_generic::<_, i32x8, I2>(x, x);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle_generic::<_, i32x8, I4>(x, x);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
        simd_shuffle_generic::<_, i32x2, I8>(x, x);
// { dg-error ".E0511." "" { target *-*-* } .-1 }
    }
}

