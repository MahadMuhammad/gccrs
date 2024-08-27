//@ aux-build:test-macros.rs
//@ needs-unwind proc macro panics to report errors

#[macro_use]
extern crate test_macros;

#[derive(Panic)]
// { dg-error "" "" { target *-*-* } .-1 }
struct Foo;

fn main() {}

