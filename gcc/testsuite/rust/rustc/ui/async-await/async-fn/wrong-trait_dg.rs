// { dg-additional-options "-frust-edition=2018" }

#![feature(async_closure)]

trait Foo {}

fn test(x: impl async Foo) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

