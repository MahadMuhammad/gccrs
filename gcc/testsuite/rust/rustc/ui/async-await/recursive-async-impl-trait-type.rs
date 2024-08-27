// { dg-additional-options "-frust-edition=2018" }
// Test that impl trait does not allow creating recursive types that are
// otherwise forbidden when using `async` and `await`.

async fn recursive_async_function() -> () {
// { dg-error ".E0733." "" { target *-*-* } .-1 }
    recursive_async_function().await;
}

fn main() {}

