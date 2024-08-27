//@ compile-flags: -Znext-solver
//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

trait Foo {
    async fn bar() {}
}

fn main() {}

