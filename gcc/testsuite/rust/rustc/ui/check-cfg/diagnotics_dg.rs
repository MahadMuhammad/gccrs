//@ check-pass
//@ no-auto-check-cfg
//@ revisions: cargo rustc
//@ [rustc]unset-rustc-env:CARGO_CRATE_NAME
//@ [cargo]rustc-env:CARGO_CRATE_NAME=foo
//@ compile-flags: --check-cfg=cfg(feature,values("foo")) --check-cfg=cfg(no_values)
//@ compile-flags: --check-cfg=cfg(quote,values("quote"))

#[cfg(featur)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn feature() {}

#[cfg(featur = "foo")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn feature() {}

#[cfg(featur = "fo")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn feature() {}

#[cfg(feature = "foo")]
fn feature() {}

#[cfg(no_value)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn no_values() {}

#[cfg(no_value = "foo")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn no_values() {}

#[cfg(no_values = "bar")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn no_values() {}

#[cfg(quote = "quote\"")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn no_values() {}

fn main() {}

