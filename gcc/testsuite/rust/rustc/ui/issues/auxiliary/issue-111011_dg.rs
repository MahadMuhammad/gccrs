#![feature(async_closure)]

// { dg-additional-options "-frust-edition=2021" }

fn foo<X>(x: impl FnOnce() -> Box<X>) {}
// just to make sure async closures can still be suggested for boxing.
fn bar<X>(x: Box<dyn FnOnce() -> X>) {}

fn main() {
    foo(async move || {}); // { dg-error ".E0308." "" { target *-*-* } }
    bar(async move || {}); // { dg-error ".E0308." "" { target *-*-* } }
}

