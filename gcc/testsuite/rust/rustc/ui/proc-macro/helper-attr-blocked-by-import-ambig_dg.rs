//@ aux-build:test-macros.rs

#[macro_use(Empty)]
extern crate test_macros;
use test_macros::empty_attr as empty_helper;

#[empty_helper] // { dg-error ".E0659." "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
#[derive(Empty)]
struct S;

fn main() {}

