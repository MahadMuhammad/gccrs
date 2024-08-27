//@ revisions: stock effects
#![feature(intrinsics)]
#![feature(rustc_attrs)]
// as effects insert a const generic param to const intrinsics,
// check here that it doesn't report a const param mismatch either
// enabling or disabling effects.
#![cfg_attr(effects, feature(effects))]
#![allow(incomplete_features)]

extern "rust-intrinsic" {
    fn size_of<T>() -> usize; // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
}

#[rustc_intrinsic]
const fn assume(_b: bool) {} // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

#[rustc_intrinsic]
const fn const_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

mod foo {
    #[rustc_intrinsic]
    unsafe fn const_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {}
    // FIXME(effects) ~^ ERROR wrong number of const parameters
}

fn main() {}

