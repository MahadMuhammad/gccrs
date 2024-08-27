//@ run-rustfix
#![allow(dead_code)]

trait Foo {
    fn bar() {}; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

