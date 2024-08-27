#![feature(coroutines, coroutine_trait)]

use std::ops::Coroutine;

fn foo() -> impl Coroutine<Return = i32> {
// { dg-error ".E0271." "" { target *-*-* } .-1 }
    #[coroutine]
    || {
        if false {
            return Ok(6);
        }

        yield ();

        5 // { dg-error ".E0308." "" { target *-*-* } }
    }
}

fn main() {}

