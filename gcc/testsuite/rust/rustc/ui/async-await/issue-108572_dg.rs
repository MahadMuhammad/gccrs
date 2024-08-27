// { dg-additional-options "-frust-edition= 2021" }
//@ run-rustfix
#![allow(unused_must_use, dead_code)]

use std::future::Future;
fn foo() -> impl Future<Output=()> {
    async { }
}

fn bar(cx: &mut std::task::Context<'_>) {
    let fut = foo();
    fut.poll(cx);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}
fn main() {}

