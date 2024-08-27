// This test check that #[allow(unexpected_cfgs)] doesn't work if put on the same level
//
//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --check-cfg=cfg()

#[allow(unexpected_cfgs)]
#[cfg(FALSE)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn bar() {}

fn main() {}

