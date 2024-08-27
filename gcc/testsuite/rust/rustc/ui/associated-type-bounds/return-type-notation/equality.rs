// { dg-additional-options "-frust-edition= 2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::future::Future;

trait Trait {
    async fn method() {}
}

fn test<T: Trait<method(..) = Box<dyn Future<Output = ()>>>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

