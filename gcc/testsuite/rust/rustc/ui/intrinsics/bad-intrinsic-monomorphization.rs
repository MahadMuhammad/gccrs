//@ build-fail

#![feature(repr_simd, intrinsics, core_intrinsics)]
#![allow(warnings)]
#![crate_type = "rlib"]

// Bad monomorphizations could previously cause LLVM asserts even though the
// error was caught in the compiler.

extern "rust-intrinsic" {
    fn simd_add<T>(x: T, y: T) -> T;
}

use std::intrinsics;

#[derive(Copy, Clone)]
pub struct Foo(i64);

pub fn test_cttz(v: Foo) -> u32 {
    intrinsics::cttz(v)
// { dg-error ".E0511." "" { target *-*-* } .-1 }
}

pub unsafe fn test_fadd_fast(a: Foo, b: Foo) -> Foo {
    intrinsics::fadd_fast(a, b)
// { dg-error ".E0511." "" { target *-*-* } .-1 }
}

pub unsafe fn test_simd_add(a: Foo, b: Foo) -> Foo {
    simd_add(a, b)
// { dg-error ".E0511." "" { target *-*-* } .-1 }
}

