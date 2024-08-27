// --force-warn $LINT_GROUP causes $LINT to warn despite
// $LINT_GROUP being allowed in module
//@ compile-flags: --force-warn rust_2018_idioms
//@ check-pass

#![allow(rust_2018_idioms)]

pub trait SomeTrait {}

pub fn function(_x: Box<SomeTrait>) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {}

