#![feature(coerce_unsized)]
use std::ops::CoerceUnsized;

struct Foo<T: ?Sized> { // { dg-error ".E0392." "" { target *-*-* } }
    a: i32,
}

impl<T, U> CoerceUnsized<Foo<U>> for Foo<T> // { dg-error ".E0374." "" { target *-*-* } }
    where T: CoerceUnsized<U> {}

fn main() {}

