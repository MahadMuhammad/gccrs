#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct Foo<T, const H: T>(T)
// { dg-error ".E0770." "" { target *-*-* } .-1 }
where
    [(); 1]:;

fn main() {}

