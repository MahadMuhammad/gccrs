// { dg-additional-options "-frust-edition=2018" }

#![feature(type_alias_impl_trait)]

use std::future::Future;

type G<'a, T> = impl Future<Output = ()>;

trait Trait {
    type F: Future<Output = ()>;

    fn f(&self) -> Self::F;

    fn g<'a>(&'a self) -> G<'a, Self>
    where
        Self: Sized,
    {
        async move { self.f().await }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }
}

fn main() {}

