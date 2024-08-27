// --force-warn $LINT casuses $LINT to warn despite --cap-lints
// set to allow
//@ compile-flags: --cap-lints allow  --force-warn bare_trait_objects
//@ check-pass

pub trait SomeTrait {}

pub fn function(_x: Box<SomeTrait>) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {}

