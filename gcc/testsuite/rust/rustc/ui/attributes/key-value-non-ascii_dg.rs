#![feature(rustc_attrs)]

#[rustc_dummy = b"ﬃ.rs"] // { dg-error "" "" { target *-*-* } }
fn main() {}

