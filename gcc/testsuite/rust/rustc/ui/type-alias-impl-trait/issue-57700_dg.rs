#![feature(arbitrary_self_types)]
#![feature(impl_trait_in_assoc_type)]

use std::ops::Deref;

trait Foo {
    type Bar: Foo;

    fn foo(self: impl Deref<Target = Self>) -> Self::Bar;
}

impl<C> Foo for C {
    type Bar = impl Foo;

    fn foo(self: impl Deref<Target = Self>) -> Self::Bar {
        self
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

