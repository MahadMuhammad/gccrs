// { dg-additional-options "-frust-edition= 2021" }

trait Foo {
    async fn foo<T>();
}

impl Foo for () {
    async fn foo<const N: usize>() {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

fn main() {}

