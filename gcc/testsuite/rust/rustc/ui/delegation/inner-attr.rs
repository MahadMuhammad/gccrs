#![feature(fn_delegation)]
#![allow(incomplete_features)]

fn a() {}

reuse a as b { #![rustc_dummy] self } // { dg-error "" "" { target *-*-* } }

fn main() {}

