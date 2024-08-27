#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

use std::fmt::Debug;

type FooX = impl Debug;
// { dg-error "" "" { target *-*-* } .-1 }

trait Foo<A> {}

impl Foo<FooX> for () {}

fn foo() -> impl Foo<FooX> {
// { dg-error "" "" { target *-*-* } .-1 }
    ()
}

fn main() {}

