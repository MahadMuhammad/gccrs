// { dg-additional-options "-frust-edition=2021" }
//@ check-pass

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Super<'a> {
    async fn test();
}
impl Super<'_> for () {
    async fn test() {}
}

trait Foo: for<'a> Super<'a> {}
impl Foo for () {}

fn test<T>()
where
    T: Foo<test(..): Send>,
{
}

fn main() {
    test::<()>();
}

