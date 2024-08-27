//@ run-pass
#![allow(dead_code)]
#![allow(dead_code)]

#![feature(coroutines)]

fn bar<'a>() {
    let a: &'static str = "hi";
    let b: &'a str = a;

    #[coroutine] || { // { dg-warning "" "" { target *-*-* } }
        yield a;
        yield b;
    };
}

fn main() {}

