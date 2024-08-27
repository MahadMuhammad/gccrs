// { dg-additional-options "-frust-edition=2021" }
//@ run-rustfix

#![allow(unused)]

use std::future::Future;

async fn foo() -> Result<(), i32> {
    func(async { Ok::<_, i32>(()) })?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    Ok(())
}

async fn func<T>(fut: impl Future<Output = T>) -> T {
    fut.await
}

fn main() {}

