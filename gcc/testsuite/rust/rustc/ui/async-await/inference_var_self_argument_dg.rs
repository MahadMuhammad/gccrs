//! This is a regression test for an ICE.
// { dg-additional-options "-frust-edition= 2021" }

trait Foo {
    async fn foo(self: &dyn Foo) {
// { dg-error ".E0307." "" { target *-*-* } .-1 }
// { dg-error ".E0307." "" { target *-*-* } .-2 }
        todo!()
    }
}

fn main() {}

