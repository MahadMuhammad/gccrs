#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
#![allow(dead_code)]

#[derive(PartialEq, Eq)]
struct U;

struct S<const N: U>()
where
    S<{ U }>:;
// { dg-error ".E0275." "" { target *-*-* } .-1 }

fn main() {}

