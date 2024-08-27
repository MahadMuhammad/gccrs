//@ only-x86_64

#![feature(target_feature_11)]

#[target_feature(enable = "avx2")]
fn main() {}
// { dg-error "" "" { target *-*-* } .-1 }

