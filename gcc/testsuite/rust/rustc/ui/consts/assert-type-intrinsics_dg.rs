#![feature(never_type)]
#![feature(core_intrinsics)]

use std::intrinsics;

#[allow(invalid_value)]
fn main() {
    use std::mem::MaybeUninit;

    const _BAD1: () = unsafe {
        MaybeUninit::<!>::uninit().assume_init();
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const _BAD2: () = {
        intrinsics::assert_mem_uninitialized_valid::<&'static i32>();
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
    const _BAD3: () = {
        intrinsics::assert_zero_valid::<&'static i32>();
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    };
}

