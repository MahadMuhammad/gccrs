#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Foo;

impl Foo {
    type Baz; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

