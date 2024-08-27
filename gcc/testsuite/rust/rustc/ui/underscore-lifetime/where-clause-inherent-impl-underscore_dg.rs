//@ revisions: rust2015 rust2018
// { dg-additional-options "-frust-edition=2018" }

trait WithType<T> {}
trait WithRegion<'a> { }

struct Foo<T> {
    t: T
}

impl<T> Foo<T>
where
    T: WithRegion<'_>
// { dg-error "" "" { target *-*-* } .-1 }
{ }

fn main() {}

