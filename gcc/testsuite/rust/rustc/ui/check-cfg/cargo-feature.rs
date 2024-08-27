// This test checks that when no features are passed by Cargo we
// suggest adding some in the Cargo.toml instead of vomitting a
// list of all the expected names
//
//@ check-pass
//@ no-auto-check-cfg
//@ revisions: some none
//@ rustc-env:CARGO_CRATE_NAME=foo
//@ [none]compile-flags: --check-cfg=cfg(feature,values())
//@ [some]compile-flags: --check-cfg=cfg(feature,values("bitcode"))
//@ [some]compile-flags: --check-cfg=cfg(CONFIG_NVME,values("y"))
//@ [none]error-pattern:Cargo.toml

#[cfg(feature = "serde")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn ser() {}

#[cfg(feature)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn feat() {}

#[cfg(tokio_unstable)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn tokio() {}

#[cfg(CONFIG_NVME = "m")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
fn tokio() {}

fn main() {}

