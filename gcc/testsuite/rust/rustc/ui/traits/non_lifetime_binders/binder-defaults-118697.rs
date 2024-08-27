#![allow(incomplete_features)]
#![feature(non_lifetime_binders)]

type T = dyn for<V = A(&())> Fn(());
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
// { dg-error ".E0412." "" { target *-*-* } .-3 }

fn main() {}

