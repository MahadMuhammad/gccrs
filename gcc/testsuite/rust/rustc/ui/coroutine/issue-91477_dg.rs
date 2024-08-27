#![feature(coroutines)]

fn foo() -> impl Sized {
    yield 1; // { dg-error ".E0627." "" { target *-*-* } }
// { dg-error ".E0627." "" { target *-*-* } .-1 }
}

fn main() {}

