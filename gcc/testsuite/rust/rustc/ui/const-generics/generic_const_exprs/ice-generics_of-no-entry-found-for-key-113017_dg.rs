// test for ICE "no entry found for key" in generics_of.rs #113017

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub fn foo()
where
    for<const N: usize = { || {}; 1 }> ():,
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
{}

pub fn main() {}

