//@ aux-build:block-on.rs
// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

extern crate block_on;

fn main() {
    block_on::block_on(async {
        let c = async |x| {};
        c(1i32).await;
        c(2usize).await;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    });
}

