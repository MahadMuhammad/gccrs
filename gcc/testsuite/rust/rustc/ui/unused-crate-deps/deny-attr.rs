// Check for unused crate dep, no path

// { dg-additional-options "-frust-edition=2018" }
//@ aux-crate:bar=bar.rs

#![deny(unused_crate_dependencies)]
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

