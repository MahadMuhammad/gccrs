//@ check-fail

#![deny(unknown_lints)]
#![allow(test_unstable_lint)]
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

