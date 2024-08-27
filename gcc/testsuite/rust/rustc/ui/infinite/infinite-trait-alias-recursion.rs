#![feature(trait_alias)]

trait T1 = T2;
// { dg-error ".E0391." "" { target *-*-* } .-1 }

trait T2 = T3;

trait T3 = T1 + T3;

fn main() {}

