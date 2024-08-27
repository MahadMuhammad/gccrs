//@ run-rustfix
#![allow(dead_code)]
struct Foo {
    field: i32,
}

impl Foo {
    fn foo<'a>(&self, x: &i32) -> &i32 {
        x
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

