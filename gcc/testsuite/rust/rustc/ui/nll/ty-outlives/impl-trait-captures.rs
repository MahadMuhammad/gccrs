//@ compile-flags:-Zverbose-internals

#![allow(warnings)]

trait Foo<'a> {
}

impl<'a, T> Foo<'a> for T { }

fn foo<'a, T>(x: &T) -> impl Foo<'a> {
    x
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn main() {}

