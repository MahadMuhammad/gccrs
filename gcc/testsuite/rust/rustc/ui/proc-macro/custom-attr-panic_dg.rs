//@ aux-build: test-macros.rs

extern crate test_macros;

#[test_macros::panic_attr] // { dg-error "" "" { target *-*-* } }
fn foo() {}

fn main() {}

