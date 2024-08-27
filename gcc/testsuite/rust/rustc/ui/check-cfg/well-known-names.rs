// This test checks that we lint on non well known names and that we don't lint on well known names
//
//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --check-cfg=cfg()

#[cfg(target_oz = "linux")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn target_os_misspell() {}

#[cfg(target_os = "linux")]
fn target_os() {}

#[cfg(features = "foo")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn feature_misspell() {}

#[cfg(feature = "foo")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn feature() {}

#[cfg(uniw)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn unix_misspell() {}

#[cfg(unix)]
fn unix() {}

#[cfg(miri)]
fn miri() {}

#[cfg(doc)]
fn doc() {}

fn main() {}

