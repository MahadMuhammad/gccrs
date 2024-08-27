// { dg-additional-options "-frust-edition= 2021" }
//@ compile-flags: --extern issue_85992_extern --extern empty
//@ aux-build: issue-85992-extern.rs
//@ aux-build: empty.rs

issue_85992_extern::m!();

use crate::empty;
// { dg-error ".E0432." "" { target *-*-* } .-1 }

fn main() {}

