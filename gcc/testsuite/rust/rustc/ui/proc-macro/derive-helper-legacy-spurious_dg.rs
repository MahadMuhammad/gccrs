//@ aux-build:test-macros.rs

#![dummy] // { dg-error "" "" { target *-*-* } }

#[macro_use]
extern crate test_macros;

#[derive(Empty)]
#[empty_helper] // { dg-error "" "" { target *-*-* } }
struct Foo {}

fn main() {}

