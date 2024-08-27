//@ check-pass
//
//@ no-auto-check-cfg
//@ revisions: explicit implicit simple concat_1 concat_2
//@ [explicit]compile-flags: --check-cfg=cfg(foo,values(none()))
//@ [implicit]compile-flags: --check-cfg=cfg(foo)
//@ [simple]  compile-flags: --check-cfg=cfg(foo,values(none(),"too"))
//@ [concat_1]compile-flags: --check-cfg=cfg(foo) --check-cfg=cfg(foo,values("too"))
//@ [concat_2]compile-flags: --check-cfg=cfg(foo,values("too")) --check-cfg=cfg(foo)

#[cfg(foo = "too")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
fn foo_too() {}

#[cfg(foo = "bar")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn foo_bar() {}

#[cfg(foo)]
fn foo() {}

fn main() {}

