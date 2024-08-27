// { dg-additional-options "-frust-edition=2018" }
//
// Tests that the .await syntax can't be used to make a coroutine

async fn foo() {}

fn make_coroutine() {
    let _gen = || foo().await;
// { dg-error ".E0728." "" { target *-*-* } .-1 }
}

fn main() {}

