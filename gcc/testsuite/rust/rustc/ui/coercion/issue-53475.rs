#![feature(coerce_unsized)]

use std::any::Any;
use std::ops::CoerceUnsized;

struct Foo<T: ?Sized> {
    data: Box<T>,
}

impl<T> CoerceUnsized<Foo<dyn Any>> for Foo<T> {}
// { dg-error ".E0310." "" { target *-*-* } .-1 }

fn main() {}

