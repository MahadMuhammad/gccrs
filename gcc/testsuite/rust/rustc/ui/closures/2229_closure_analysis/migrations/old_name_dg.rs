//@ check-pass

// Ensure that the old name for `rust_2021_incompatible_closure_captures` is still
// accepted by the compiler

#![allow(disjoint_capture_migration)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {}

