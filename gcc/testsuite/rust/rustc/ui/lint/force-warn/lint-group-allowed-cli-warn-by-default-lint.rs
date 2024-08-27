// --force-warn $LINT_GROUP causes $LINT (which is warn-by-default) to warn
// despite $LINT being allowed on command line
//@ compile-flags: -A bare-trait-objects --force-warn rust-2018-idioms
//@ check-pass

pub trait SomeTrait {}

pub fn function(_x: Box<SomeTrait>) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {}

