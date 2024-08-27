#![feature(repr_simd)]
#![allow(non_camel_case_types)]


#[repr(simd)]
struct empty; // { dg-error ".E0075." "" { target *-*-* } }

#[repr(simd)]
struct empty2([f32; 0]); // { dg-error ".E0075." "" { target *-*-* } }

#[repr(simd)]
struct pow2([f32; 7]);

#[repr(simd)]
struct i64f64(i64, f64); // { dg-error ".E0076." "" { target *-*-* } }

struct Foo;

#[repr(simd)]
struct FooV(Foo, Foo); // { dg-error ".E0077." "" { target *-*-* } }

#[repr(simd)]
struct FooV2([Foo; 2]); // { dg-error ".E0077." "" { target *-*-* } }

#[repr(simd)]
struct TooBig([f32; 65536]); // { dg-error ".E0075." "" { target *-*-* } }

#[repr(simd)]
struct JustRight([u128; 32768]);

#[repr(simd)]
struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

fn main() {}

