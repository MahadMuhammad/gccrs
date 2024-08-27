// { dg-additional-options "-frust-edition=2021" }
// https://github.com/rust-lang/rust/issues/117547

trait T {}

trait MyTrait {
    async fn foo() -> &'static impl T;
// { dg-error ".E0310." "" { target *-*-* } .-1 }
}

fn main() {}

