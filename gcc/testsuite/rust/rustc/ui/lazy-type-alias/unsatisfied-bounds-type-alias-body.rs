// Test that we check lazy type aliases for well-formedness.

#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Alias<T> = <T as std::ops::Mul>::Output; // { dg-error ".E0277." "" { target *-*-* } }

fn main() {}

