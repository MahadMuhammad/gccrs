//@ run-rustfix

// check to make sure that we suggest adding `move` after `static`

#![feature(coroutines)]

fn check() -> impl Sized {
    let x = 0;
    #[coroutine]
    static || {
// { dg-error ".E0373." "" { target *-*-* } .-1 }
        yield;
        x
    }
}

fn main() {
    check();
}

