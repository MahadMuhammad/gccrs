// { dg-additional-options "-frust-edition=2018" }

#![feature(impl_trait_in_assoc_type)]

use std::future::Future;

trait MakeFut {
    type Fut<'a>
    where
        Self: 'a;
    fn make_fut<'a>(&'a self) -> Self::Fut<'a>;
}

impl MakeFut for &'_ mut () {
    type Fut<'a> = impl Future<Output = ()>;
// { dg-error ".E0477." "" { target *-*-* } .-1 }

    fn make_fut<'a>(&'a self) -> Self::Fut<'a> {
        async { () }
    }
}

fn main() {}

