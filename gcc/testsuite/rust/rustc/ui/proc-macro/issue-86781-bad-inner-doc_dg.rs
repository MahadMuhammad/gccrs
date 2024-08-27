//@ aux-build:test-macros.rs
//@ run-rustfix

#[macro_use]
extern crate test_macros;

//! Inner doc comment
// { dg-error ".E0753." "" { target *-*-* } .-1 }
#[derive(Empty)]
pub struct Foo; // { dg-note "" "" { target *-*-* } }

fn main() {}

