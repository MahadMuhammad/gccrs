#![feature(coroutines)]

fn main() { yield; }
// { dg-error ".E0627." "" { target *-*-* } .-1 }
// { dg-error ".E0627." "" { target *-*-* } .-2 }

