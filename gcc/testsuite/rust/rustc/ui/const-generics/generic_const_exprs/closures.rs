#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
// { dg-error ".E0391." "" { target *-*-* } .-1 }

fn main() {}

