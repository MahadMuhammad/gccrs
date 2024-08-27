#![feature(cfg_accessible)]

#[cfg_accessible(Z)] // OK, recovered after the other `cfg_accessible` produces an error.
struct S;

#[cfg_accessible(S)] // { dg-error "" "" { target *-*-* } }
struct Z;

fn main() {}

