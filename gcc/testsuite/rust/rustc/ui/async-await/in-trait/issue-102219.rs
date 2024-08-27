//@ compile-flags:--crate-type=lib
// { dg-additional-options "-frust-edition=2021" }
//@ check-pass

trait T {
    #[allow(async_fn_in_trait)]
    async fn foo();
}

