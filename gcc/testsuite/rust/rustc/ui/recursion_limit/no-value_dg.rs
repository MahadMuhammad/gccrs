// Test the parse error for no value provided to recursion_limit

#![recursion_limit]
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

