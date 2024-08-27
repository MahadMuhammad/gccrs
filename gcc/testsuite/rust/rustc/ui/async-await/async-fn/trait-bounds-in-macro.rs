// { dg-additional-options "-frust-edition= 2021" }

macro_rules! x {
    ($x:item) => {}
}

x! {
    async fn foo() -> impl async Fn() { }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

fn main() {}

