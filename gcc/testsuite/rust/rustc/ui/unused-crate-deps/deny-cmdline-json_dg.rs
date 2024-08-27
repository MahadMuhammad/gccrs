// Check for unused crate dep, json event, deny, expect compile failure

// { dg-additional-options "-frust-edition=2018" }
//@ compile-flags: -Dunused-crate-dependencies --json unused-externs --error-format=json
//@ aux-crate:bar=bar.rs

fn main() {}

