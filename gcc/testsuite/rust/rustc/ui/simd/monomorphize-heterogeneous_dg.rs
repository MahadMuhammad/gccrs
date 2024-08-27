#![feature(repr_simd)]

#[repr(simd)]
struct I64F64(i64, f64);
// { dg-error ".E0076." "" { target *-*-* } .-1 }

static X: I64F64 = I64F64(1, 2.0);

fn main() {}

