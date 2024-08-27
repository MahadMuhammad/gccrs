//@ compile-flags: -Znext-solver=coherence

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub struct A<const z: [usize; x]> {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }

impl A<2> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    pub const fn B() {}
// { dg-error ".E0592." "" { target *-*-* } .-1 }
}

impl A<2> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    pub const fn B() {}
}

fn main() {}

