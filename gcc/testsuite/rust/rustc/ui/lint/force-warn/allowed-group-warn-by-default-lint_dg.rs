// --force-warn $LINT causes $LINT (which is warn-by-default) to warn
// despite $LINT_GROUP (which contains $LINT) being allowed
//@ compile-flags: --force-warn bare_trait_objects
//@ check-pass

#![allow(rust_2018_idioms)]

pub trait SomeTrait {}

pub fn function(_x: Box<SomeTrait>) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {}

