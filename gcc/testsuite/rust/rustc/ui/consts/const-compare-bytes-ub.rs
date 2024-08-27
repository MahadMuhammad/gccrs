//@ check-fail

#![feature(core_intrinsics)]
#![feature(const_intrinsic_compare_bytes)]
use std::intrinsics::compare_bytes;
use std::mem::MaybeUninit;

fn main() {
    const LHS_NULL: i32 = unsafe {
        compare_bytes(0 as *const u8, 2 as *const u8, 1)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const RHS_NULL: i32 = unsafe {
        compare_bytes(1 as *const u8, 0 as *const u8, 1)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const DANGLING_PTR_NON_ZERO_LENGTH: i32 = unsafe {
        compare_bytes(1 as *const u8, 2 as *const u8, 1)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const LHS_OUT_OF_BOUNDS: i32 = unsafe {
        compare_bytes([1, 2, 3].as_ptr(), [1, 2, 3, 4].as_ptr(), 4)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const RHS_OUT_OF_BOUNDS: i32 = unsafe {
        compare_bytes([1, 2, 3, 4].as_ptr(), [1, 2, 3].as_ptr(), 4)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const LHS_UNINIT: i32 = unsafe {
        compare_bytes(MaybeUninit::uninit().as_ptr(), [1].as_ptr(), 1)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const RHS_UNINIT: i32 = unsafe {
        compare_bytes([1].as_ptr(), MaybeUninit::uninit().as_ptr(), 1)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const WITH_PROVENANCE: i32 = unsafe {
        compare_bytes([&1].as_ptr().cast(), [&2].as_ptr().cast(), std::mem::size_of::<usize>())
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
}

