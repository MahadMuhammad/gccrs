//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --check-cfg=cfg(my_cfg,values("foo")) --check-cfg=cfg(my_cfg,values("bar"))
//@ compile-flags: --check-cfg=cfg(my_cfg,values())

#[cfg(my_cfg)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn my_cfg() {}

#[cfg(my_cfg = "unk")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn my_cfg() {}

#[cfg(any(my_cfg = "foo", my_cfg = "bar"))]
fn foo_and_bar() {}

fn main() {}

