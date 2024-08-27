//@ run-rustfix

#![allow(dead_code)]
trait Foo {
    fn foo([a, b]: [i32; 2]) {}
// { dg-error ".E0642." "" { target *-*-* } .-1 }
}

fn main() {}

