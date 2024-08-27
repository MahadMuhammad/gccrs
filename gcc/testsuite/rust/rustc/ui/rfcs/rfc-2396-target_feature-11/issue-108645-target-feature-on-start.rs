//@ only-x86_64

#![feature(start)]
#![feature(target_feature_11)]

#[start]
#[target_feature(enable = "avx2")]
// { dg-error "" "" { target *-*-* } .-1 }
fn start(_argc: isize, _argv: *const *const u8) -> isize { 0 }

