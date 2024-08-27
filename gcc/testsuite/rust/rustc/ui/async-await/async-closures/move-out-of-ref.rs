//@ compile-flags: -Zvalidate-mir
// { dg-additional-options "-frust-edition= 2021" }

#![feature(async_closure)]

// NOT copy.
struct Ty;

fn hello(x: &Ty) {
    let c = async || {
        *x;
// { dg-error ".E0507." "" { target *-*-* } .-1 }
    };
}

fn main() {}

