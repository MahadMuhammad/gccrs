// check that `pattern_complexity` is feature-gated

#![pattern_complexity = "42"]
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

