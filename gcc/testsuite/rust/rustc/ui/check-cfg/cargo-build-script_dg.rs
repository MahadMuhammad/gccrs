// This test checks that when we are building a build script provided
// by Cargo we only suggest expecting the unexpected cfgs in the Cargo.toml.
//
//@ check-pass
//@ no-auto-check-cfg
//@ rustc-env:CARGO_CRATE_NAME=build_script_build
//@ compile-flags:--crate-name=build_script_build
//@ compile-flags:--check-cfg=cfg(has_bar)

#[cfg(has_foo)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn foo() {}

#[cfg(has_foo = "yes")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn foo() {}

#[cfg(has_bar = "yes")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn has_bar() {}

fn main() {}

