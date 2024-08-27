//@ compile-flags: -Cpanic=unwind  --crate-type=lib
//@ no-prefer-dynamic
// { dg-additional-options "-frust-edition=2021" }

#![feature(coroutines, stmt_expr_attributes)]
pub fn run<T>(a: T) {
    let _ = #[coroutine]
    move || {
        drop(a);
        yield;
    };
}

