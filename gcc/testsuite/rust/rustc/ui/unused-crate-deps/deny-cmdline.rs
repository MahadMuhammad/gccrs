// Check for unused crate dep, deny, expect failure

// { dg-additional-options "-frust-edition=2018" }
//@ compile-flags: -Dunused-crate-dependencies
//@ aux-crate:bar=bar.rs

fn main() {}
// { dg-error "" "" { target *-*-* } .-1 }

