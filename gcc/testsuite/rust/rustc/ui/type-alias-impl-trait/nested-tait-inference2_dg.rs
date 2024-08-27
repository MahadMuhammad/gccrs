#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

use std::fmt::Debug;

type FooX = impl Debug;

trait Foo<A> {}

impl Foo<()> for () {}
impl Foo<u32> for () {}

fn foo() -> impl Foo<FooX> {
// { dg-error "" "" { target *-*-* } .-1 }
    ()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

