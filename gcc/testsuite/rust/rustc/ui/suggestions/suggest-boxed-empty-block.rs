#![feature(async_closure)]

// { dg-additional-options "-frust-edition=2021" }
//@ run-rustfix

fn foo<T>(_: Box<T>) {}
fn bar<T>(_: impl Fn() -> Box<T>) {}

fn main() {
    foo({}); // { dg-error ".E0308." "" { target *-*-* } }
    bar(|| {}); // { dg-error ".E0308." "" { target *-*-* } }
}

