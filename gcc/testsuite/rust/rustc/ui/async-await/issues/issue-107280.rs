// { dg-additional-options "-frust-edition=2021" }

async fn foo() {
    inner::<false>().await
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

async fn inner<T, const PING: bool>() {}

fn main() {}

