#![derive(Debug, PartialEq, Eq)] // should be an outer attribute!
// { dg-error "" "" { target *-*-* } .-1 }
struct DerivedOn;

fn main() {}

