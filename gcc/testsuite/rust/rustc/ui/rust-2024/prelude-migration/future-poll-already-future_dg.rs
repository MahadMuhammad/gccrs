//@ revisions: e2021 e2024
// { dg-additional-options "-frust-edition= 2021" }
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options
//@ check-pass

#![deny(rust_2024_prelude_collisions)]

use std::future::Future;

fn main() {
    core::pin::pin!(async {}).poll(&mut context());
}

fn context() -> core::task::Context<'static> {
    loop {}
}

