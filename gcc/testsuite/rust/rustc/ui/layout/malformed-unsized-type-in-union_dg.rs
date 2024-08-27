// issue: 113760

union W { s: dyn Iterator<Item = Missing> }
// { dg-error ".E0412." "" { target *-*-* } .-1 }

static ONCE: W = todo!();

fn main() {}

