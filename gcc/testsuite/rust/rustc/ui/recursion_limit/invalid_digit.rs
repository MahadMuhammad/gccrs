// Test the parse error for an invalid digit in recursion_limit

#![recursion_limit = "-100"] // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
fn main() {}

