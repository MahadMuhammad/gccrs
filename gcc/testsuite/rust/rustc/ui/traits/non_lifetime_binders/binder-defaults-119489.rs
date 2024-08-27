#![feature(non_lifetime_binders, generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn fun()
where
    for<T = (), const N: usize = 1> ():,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
{}

fn main() {}

