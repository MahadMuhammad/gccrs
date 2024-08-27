#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct S<const S: (), const S: S = { S }>;
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-error ".E0391." "" { target *-*-* } .-2 }
// { dg-error ".E0391." "" { target *-*-* } .-3 }
// { dg-error ".E0391." "" { target *-*-* } .-4 }
// { dg-error ".E0391." "" { target *-*-* } .-5 }

fn main() {}

