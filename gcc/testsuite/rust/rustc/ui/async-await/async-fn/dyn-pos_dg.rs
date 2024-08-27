// { dg-additional-options "-frust-edition=2018" }

#![feature(async_closure)]

fn foo(x: &dyn async Fn()) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }

fn main() {}

