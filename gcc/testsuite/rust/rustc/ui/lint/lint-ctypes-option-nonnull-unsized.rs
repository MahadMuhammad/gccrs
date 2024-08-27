#![deny(improper_ctypes_definitions)]

extern "C" fn foo<T: ?Sized + 'static>() -> Option<&'static T> {
// { dg-error "" "" { target *-*-* } .-1 }
    None
}

fn main() {}

