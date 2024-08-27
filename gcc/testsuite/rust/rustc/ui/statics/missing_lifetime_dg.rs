//! This test checks that we taint typeck results when there are
//! error lifetimes, even though typeck doesn't actually care about lifetimes.

struct Slice(&'reborrow [&'static [u8]]);
// { dg-error ".E0261." "" { target *-*-* } .-1 }

static MAP: Slice = Slice(&[b"" as &'static [u8]]);

fn main() {}

