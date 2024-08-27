#![feature(coroutine_trait)]
#![feature(coroutines)]
#![feature(unsized_locals)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::ops::Coroutine;

fn capture() -> impl Coroutine {
    let b: [u8] = *(Box::new([]) as Box<[u8]>);
    #[coroutine]
    move || {
        println!("{:?}", &b);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

        yield;

        for elem in b.iter() {}
    }
}

fn main() {
    capture();
}

