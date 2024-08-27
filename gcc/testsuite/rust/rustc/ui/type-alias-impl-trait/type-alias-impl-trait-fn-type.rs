#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

// FIXME: this is ruled out for now but should work

type Foo = fn() -> impl Send;
// { dg-error ".E0562." "" { target *-*-* } .-1 }

fn make_foo() -> Foo {
    || 15
}

fn main() {}

