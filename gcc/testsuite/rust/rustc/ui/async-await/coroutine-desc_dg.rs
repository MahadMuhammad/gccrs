// { dg-additional-options "-frust-edition=2018" }
#![feature(async_closure)]
use std::future::Future;

async fn one() {}
async fn two() {}

fn fun<F: Future<Output = ()>>(f1: F, f2: F) {}
fn main() {
    fun(async {}, async {});
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    fun(one(), two());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    fun((async || {})(), (async || {})());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

