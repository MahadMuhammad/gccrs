//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub fn f<T>() where for<U> (T, U): Copy {}

