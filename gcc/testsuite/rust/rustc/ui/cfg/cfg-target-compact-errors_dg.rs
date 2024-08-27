//@ check-fail

#![feature(cfg_target_compact)]

#[cfg(target(o::o))]
// { dg-error "" "" { target *-*-* } .-1 }
fn one() {}

#[cfg(target(os = 8))]
// { dg-error ".E0565." "" { target *-*-* } .-1 }
fn two() {}

#[cfg(target(os = "linux", pointer(width = "64")))]
// { dg-error ".E0537." "" { target *-*-* } .-1 }
fn three() {}

fn main() {}

