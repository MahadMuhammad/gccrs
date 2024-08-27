use std::marker::SmartPointer; // { dg-error ".E0658." "" { target *-*-* } }

#[derive(SmartPointer)] // { dg-error ".E0658." "" { target *-*-* } }
#[repr(transparent)]
struct MyPointer<'a, #[pointee] T: ?Sized> {
    ptr: &'a T,
}

fn main() {}

