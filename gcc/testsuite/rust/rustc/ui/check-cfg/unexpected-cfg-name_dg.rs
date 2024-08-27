// Check warning for unexpected configuration name
//
//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --check-cfg=cfg()

#[cfg(widnows)]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn f() {}

#[cfg(windows)]
pub fn g() {}

pub fn main() {}

