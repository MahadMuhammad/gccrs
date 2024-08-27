#![feature(min_specialization)]

struct S;

impl From<S> for S {
    fn from(s: S) -> S { // { dg-error ".E0520." "" { target *-*-* } }
        s
    }
}

fn main() {}

