// Check for unused crate dep, no path

// { dg-additional-options "-frust-edition=2018" }
//@ check-pass
//@ compile-flags: -Wunused-crate-dependencies
//@ aux-crate:bar=bar.rs

fn main() {}
// { dg-warning "" "" { target *-*-* } .-1 }

