//@ aux-build:block-on.rs
// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

extern crate block_on;

struct NoCopy;

fn main() {
    block_on::block_on(async {
        let s = NoCopy;
        let x = async move || {
            drop(s);
        };
        x().await;
        x().await;
// { dg-error ".E0382." "" { target *-*-* } .-1 }
    });
}

