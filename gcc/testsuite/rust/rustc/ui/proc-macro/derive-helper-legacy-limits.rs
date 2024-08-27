// Support for legacy derive helpers is limited and heuristic-based
// (that's exactly the reason why they are deprecated).

// { dg-additional-options "-frust-edition=2018" }
//@ aux-build:test-macros.rs

#[macro_use]
extern crate test_macros;

use derive as my_derive;

#[my_derive(Empty)]
#[empty_helper] // OK
struct S1;

// Legacy helper detection doesn't see through `derive` renaming.
#[empty_helper] // { dg-error "" "" { target *-*-* } }
#[my_derive(Empty)]
struct S2;

fn main() {}

