//@ compile-flags: --crate-type=lib
// This test checks that the `where_clauses_object_safety` lint does not cause
// other object safety *hard errors* to be suppressed, because we currently
// only emit one object safety error per trait...

use std::future::Future;
use std::pin::Pin;

pub trait Fetcher: Send + Sync {
    fn get<'a>(self: &'a Box<Self>) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'a>>
    where
        Self: Sync,
    {
        todo!()
    }
}

fn fetcher() -> Box<dyn Fetcher> {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
    todo!()
}

pub fn foo() {
    let fetcher = fetcher();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
    let _ = fetcher.get();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

