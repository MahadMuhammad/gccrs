#![feature(impl_trait_in_assoc_type)]

pub trait Bar {
    type E: Copy;

    fn foo<T>() -> Self::E;
}

impl<S> Bar for S {
    type E = impl Copy;

    fn foo<T>() -> Self::E {
        || ()
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

