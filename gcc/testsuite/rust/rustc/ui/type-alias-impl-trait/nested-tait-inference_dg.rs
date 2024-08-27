#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@[next] check-pass

use std::fmt::Debug;

type FooX = impl Debug;

trait Foo<A> {}

impl Foo<()> for () {}

fn foo() -> impl Foo<FooX> {
// { dg-error "" "" { target *-*-* } .-1 }
    // FIXME(type-alias-impl-trait): We could probably make this work.
    ()
}

fn main() {}

