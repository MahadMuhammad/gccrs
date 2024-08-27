// { dg-additional-options "-frust-edition= 2021" }

#![deny(dead_code)]

pub fn foo() {
    let closure = || {
        fn a() {}   // { dg-error "" "" { target *-*-* } }
    };
    closure()
}

pub async fn async_foo() {
    const A: usize = 1; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

