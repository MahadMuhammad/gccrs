// { dg-additional-options "-frust-edition= 2021" }

trait Foo {
    type T;

    async fn foo(&self) -> Self::T;
}

struct Bar;

impl Foo for Bar {
    type T = ();

    async fn foo(&self) {}
}

impl Foo for Bar {
// { dg-error ".E0119." "" { target *-*-* } .-1 }
    type T = ();

    async fn foo(&self) {}
}

fn main() {}

