#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {}
struct S;

impl Trait for u8 {
    reuse unresolved::*; // { dg-error ".E0433." "" { target *-*-* } }
    reuse S::*; // { dg-error "" "" { target *-*-* } }
}

fn main() {}

