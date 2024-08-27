// Check warning for unexpected cfg
//
//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --check-cfg=cfg()

#[cfg(unknown_key = "value")]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn f() {}

fn main() {}

