#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {}

struct S;
impl S {
    reuse Trait::*; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

