#![feature(rustc_attrs)]

#![rustc_dummy=5z] // { dg-error "" "" { target *-*-* } }
fn main() {}

