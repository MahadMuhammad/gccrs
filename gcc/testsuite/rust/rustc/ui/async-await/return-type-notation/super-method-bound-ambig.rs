// { dg-additional-options "-frust-edition=2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Super1<'a> {
    async fn test();
}
impl Super1<'_> for () {
    async fn test() {}
}

trait Super2 {
    async fn test();
}
impl Super2 for () {
    async fn test() {}
}

trait Foo: for<'a> Super1<'a> + Super2 {}
impl Foo for () {}

fn test<T>()
where
    T: Foo<test(..): Send>,
// { dg-error ".E0221." "" { target *-*-* } .-1 }
{
}

fn main() {
    test::<()>();
}

