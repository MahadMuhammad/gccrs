// Check for unexpected configuration value in the code.
//
//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --cfg=feature="rand"
//@ compile-flags: --check-cfg=cfg(feature,values("serde","full"))

#[cfg(feature = "sedre")]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn f() {}

#[cfg(feature = "serde")]
pub fn g() {}

#[cfg(feature = "rand")]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn h() {}

pub fn main() {}

