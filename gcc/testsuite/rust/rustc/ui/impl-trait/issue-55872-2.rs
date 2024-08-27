// { dg-additional-options "-frust-edition=2018" }

#![feature(impl_trait_in_assoc_type)]

pub trait Bar {
    type E: Send;

    fn foo<T>() -> Self::E;
}

impl<S> Bar for S {
    type E = impl std::marker::Send;
    fn foo<T>() -> Self::E {
        async {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

fn main() {}

