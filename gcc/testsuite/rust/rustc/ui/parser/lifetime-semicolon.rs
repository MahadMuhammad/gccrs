//@ run-rustfix
#![allow(unused)]
struct Foo<'a, 'b> {
    a: &'a &'b i32
}

fn foo<'a, 'b>(_x: &mut Foo<'a; 'b>) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

