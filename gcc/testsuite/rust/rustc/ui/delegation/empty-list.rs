#![feature(fn_delegation)]
#![allow(incomplete_features)]

mod m {}

reuse m::{}; // { dg-error "" "" { target *-*-* } }

fn main() {}

