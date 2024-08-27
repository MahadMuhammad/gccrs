#![feature(fn_delegation)]
#![allow(incomplete_features)]

struct S;

impl S {
    reuse <u8>::*; // { dg-error "" "" { target *-*-* } }
    reuse <()>::*; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

