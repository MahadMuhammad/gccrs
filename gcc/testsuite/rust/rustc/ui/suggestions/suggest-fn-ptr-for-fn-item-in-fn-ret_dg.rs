//@ run-rustfix

#![allow(unused)]

struct Wrapper<T>(T);

fn bar() -> _ { Wrapper(foo) }
// { dg-error ".E0121." "" { target *-*-* } .-1 }

fn foo() {}

fn main() {}

