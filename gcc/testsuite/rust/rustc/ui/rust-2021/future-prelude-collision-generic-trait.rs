// See https://github.com/rust-lang/rust/issues/88470
//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
//@ check-pass
#![warn(rust_2021_prelude_collisions)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub trait PyTryFrom<'v, T>: Sized {
    fn try_from<V>(value: V) -> Result<&'v Self, T>;
}

pub trait PyTryInto<T>: Sized {
    fn try_into(&self) -> Result<&T, i32>;
}

struct Foo;

impl<U> PyTryInto<U> for Foo
where
    U: for<'v> PyTryFrom<'v, i32>,
{
    fn try_into(&self) -> Result<&U, i32> {
        U::try_from(self)
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

fn main() {}

