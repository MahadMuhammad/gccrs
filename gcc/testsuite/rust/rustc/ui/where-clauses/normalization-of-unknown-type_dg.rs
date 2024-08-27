//! This test used to ICE: rust-lang/rust#123276 because we did
//! not taint when failing to find the `Foo` type and then tried
//! to normalize it.
// { dg-additional-options "-frust-edition=2021" }

async fn create_task() {
    _ = Some(async { bind(documentation_filter()) });
}

async fn bind<Fut, F: Filter<Future = Fut>>(_: F) {}

fn documentation_filter() -> impl Filter {
    AndThen
}

trait Filter {
    type Future;
}

struct AndThen;

impl Filter for AndThen
where
    Foo: Filter, // { dg-error ".E0412." "" { target *-*-* } }
{
    type Future = ();
}

fn main() {}

