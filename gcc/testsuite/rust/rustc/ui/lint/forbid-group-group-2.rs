// Check what happens when we forbid a bigger group but
// then deny a subset of that group.

#![forbid(warnings)]
#![deny(forbidden_lint_groups)]

#[allow(nonstandard_style)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
fn main() {}

