//@ compile-flags: -Znext-solver
// { dg-additional-options "-frust-edition= 2021" }
//@ revisions: pass fail
//@[pass] check-pass

#![feature(coroutine_trait, coroutines)]

use std::ops::Coroutine;

struct A;
struct B;
struct C;

fn needs_coroutine(_: impl Coroutine<A, Yield = B, Return = C>) {}

#[cfg(fail)]
fn main() {
    needs_coroutine(
        #[coroutine]
        || {
// { dg-error "" "" { target *-*-* } .-1 }
            yield ();
        },
    );
}

#[cfg(pass)]
fn main() {
    needs_coroutine(
        #[coroutine]
        |_: A| {
            let _: A = yield B;
            C
        },
    )
}

