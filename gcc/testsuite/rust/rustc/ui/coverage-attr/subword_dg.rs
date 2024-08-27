#![feature(coverage_attribute)]
// { dg-additional-options "-frust-edition= 2021" }

// Check that yes/no in `#[coverage(yes)]` and `#[coverage(no)]` must be bare
// words, not part of a more complicated substructure.

#[coverage(yes(milord))] // { dg-error "" "" { target *-*-* } }
fn yes_list() {}

#[coverage(no(milord))] // { dg-error "" "" { target *-*-* } }
fn no_list() {}

#[coverage(yes = "milord")] // { dg-error "" "" { target *-*-* } }
fn yes_key() {}

#[coverage(no = "milord")] // { dg-error "" "" { target *-*-* } }
fn no_key() {}

fn main() {}

