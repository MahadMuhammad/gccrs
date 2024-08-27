//@ check-pass

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub fn foo()
where
    for<V> V: Sized,
{
    bar();
}

pub fn bar()
where
    for<V> V: Sized,
{
}

pub fn main() {}

