//@ normalize-stderr-test: "\d+ bits" -> "N bits"

// Tests that `transmute` cannot be called on types of different size.

#![allow(warnings)]
#![feature(specialization)]

use std::mem::transmute;

unsafe fn f() {
    let _: i8 = transmute(16i16);
// { dg-error ".E0512." "" { target *-*-* } .-1 }
}

unsafe fn g<T>(x: &T) {
    let _: i8 = transmute(x);
// { dg-error ".E0512." "" { target *-*-* } .-1 }
}

trait Specializable { type Output; }

impl<T> Specializable for T {
    default type Output = u16;
}

unsafe fn specializable<T>(x: u16) -> <T as Specializable>::Output {
    transmute(x)
// { dg-error ".E0512." "" { target *-*-* } .-1 }
}

#[repr(align(32))]
struct OverAlignZST;
pub struct PtrAndOverAlignZST<T: ?Sized> {
    _inner: *mut T,
    _other: OverAlignZST,
}
pub unsafe fn shouldnt_work<T: ?Sized>(from: *mut T) -> PtrAndOverAlignZST<T> {
    transmute(from)
// { dg-error ".E0512." "" { target *-*-* } .-1 }
}

pub struct PtrAndEmptyArray<T: ?Sized> {
    _inner: *mut T,
    _other: [*mut T; 0],
}
pub unsafe fn shouldnt_work2<T: ?Sized>(from: *mut T) -> PtrAndEmptyArray<T> {
    std::mem::transmute(from)
// { dg-error ".E0512." "" { target *-*-* } .-1 }
}

fn main() {}

