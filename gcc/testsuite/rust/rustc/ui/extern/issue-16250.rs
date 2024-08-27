#![deny(warnings)]

pub struct Foo;

extern "C" {
    pub fn foo(x: (Foo)); // { dg-error "" "" { target *-*-* } }
}

fn main() {}

