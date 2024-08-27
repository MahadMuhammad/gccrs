// { dg-additional-options "-frust-edition=2021" }

pub trait Foo {
    async fn woopsie_async(&self) -> String {
        42
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

fn main() {}

