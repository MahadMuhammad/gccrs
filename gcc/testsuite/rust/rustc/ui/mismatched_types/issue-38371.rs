//@ run-rustfix
// see also issue-38371-unfixable.rs
#![allow(dead_code)]

#[derive(Copy, Clone)]
struct Foo {}

fn foo(&_a: Foo) {} // { dg-error ".E0308." "" { target *-*-* } }

fn bar(_a: Foo) {}

fn qux(_a: &Foo) {}

fn zar(&_a: &Foo) {}

fn agh(&&_a: &u32) {} // { dg-error ".E0308." "" { target *-*-* } }

fn main() {}

