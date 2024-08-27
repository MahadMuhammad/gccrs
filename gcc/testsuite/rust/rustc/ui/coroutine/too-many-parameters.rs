#![feature(coroutines)]

fn main() {
    #[coroutine]
    |(), ()| {
// { dg-error ".E0628." "" { target *-*-* } .-1 }
        yield;
    };
}

