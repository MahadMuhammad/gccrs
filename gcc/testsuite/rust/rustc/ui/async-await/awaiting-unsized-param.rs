// { dg-additional-options "-frust-edition= 2021" }

#![feature(unsized_fn_params, unsized_locals)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::future::Future;

async fn bug<T>(mut f: dyn Future<Output = T> + Unpin) -> T {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    (&mut f).await
}

fn main() {}

