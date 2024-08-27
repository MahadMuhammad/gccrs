//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }

#![allow(dead_code)]

trait WithType<T> {}
trait WithRegion<'a> { }

trait Foo { }

impl<T> Foo for Vec<T>
where
    T: WithType<&u32>
// { dg-error ".E0637." "" { target *-*-* } .-1 }
{ }

fn main() {}

