//@ compile-flags: -Znext-solver

#[derive(Debug)]
struct X<const FN: fn() = { || {} }>;
// { dg-error ".E0284." "" { target *-*-* } .-1 }
// { dg-error ".E0284." "" { target *-*-* } .-2 }
// { dg-error ".E0284." "" { target *-*-* } .-3 }

fn main() {}

