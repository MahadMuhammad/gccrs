//@ compile-flags: -Znext-solver

#![feature(negative_bounds, unboxed_closures)]

fn produce() -> impl !Fn<(u32,)> {}
// { dg-error ".E0271." "" { target *-*-* } .-1 }

fn main() {}

