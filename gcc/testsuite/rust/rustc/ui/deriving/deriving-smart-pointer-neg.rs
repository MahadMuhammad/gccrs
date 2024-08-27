#![feature(derive_smart_pointer, arbitrary_self_types)]

extern crate core;
use std::marker::SmartPointer;

#[derive(SmartPointer)]
// { dg-error "" "" { target *-*-* } .-1 }
enum NotStruct<'a, T: ?Sized> {
    Variant(&'a T),
}

#[derive(SmartPointer)]
// { dg-error "" "" { target *-*-* } .-1 }
#[repr(transparent)]
struct NoField<'a, #[pointee] T: ?Sized> {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }

#[derive(SmartPointer)]
// { dg-error "" "" { target *-*-* } .-1 }
#[repr(transparent)]
struct NoFieldUnit<'a, #[pointee] T: ?Sized>();
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }

#[derive(SmartPointer)]
// { dg-error "" "" { target *-*-* } .-1 }
#[repr(transparent)]
struct NoGeneric<'a>(&'a u8);

#[derive(SmartPointer)]
// { dg-error "" "" { target *-*-* } .-1 }
#[repr(transparent)]
struct AmbiguousPointee<'a, T1: ?Sized, T2: ?Sized> {
    a: (&'a T1, &'a T2),
}

#[derive(SmartPointer)]
#[repr(transparent)]
struct TooManyPointees<'a, #[pointee] A: ?Sized, #[pointee] B: ?Sized>((&'a A, &'a B));
// { dg-error "" "" { target *-*-* } .-1 }

#[derive(SmartPointer)]
// { dg-error "" "" { target *-*-* } .-1 }
struct NotTransparent<'a, #[pointee] T: ?Sized> {
    ptr: &'a T,
}

#[derive(SmartPointer)]
#[repr(transparent)]
struct NoMaybeSized<'a, #[pointee] T> {
// { dg-error "" "" { target *-*-* } .-1 }
    ptr: &'a T,
}

// However, reordering attributes should work nevertheless.
#[repr(transparent)]
#[derive(SmartPointer)]
struct ThisIsAPossibleSmartPointer<'a, #[pointee] T: ?Sized> {
    ptr: &'a T,
}

// Also, these paths to Sized should work
#[derive(SmartPointer)]
#[repr(transparent)]
struct StdSized<'a, #[pointee] T: ?std::marker::Sized> {
    ptr: &'a T,
}
#[derive(SmartPointer)]
#[repr(transparent)]
struct CoreSized<'a, #[pointee] T: ?core::marker::Sized> {
    ptr: &'a T,
}
#[derive(SmartPointer)]
#[repr(transparent)]
struct GlobalStdSized<'a, #[pointee] T: ?::std::marker::Sized> {
    ptr: &'a T,
}
#[derive(SmartPointer)]
#[repr(transparent)]
struct GlobalCoreSized<'a, #[pointee] T: ?::core::marker::Sized> {
    ptr: &'a T,
}

fn main() {}

