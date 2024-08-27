// check that `move_size_limit is feature-gated

#![move_size_limit = "42"] // { dg-error ".E0658." "" { target *-*-* } }

fn main() {}

