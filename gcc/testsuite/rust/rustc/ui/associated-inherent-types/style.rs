#![feature(inherent_associated_types)]
#![allow(incomplete_features, dead_code)]
#![deny(non_camel_case_types)]

struct S;

impl S {
    type typ = ();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

