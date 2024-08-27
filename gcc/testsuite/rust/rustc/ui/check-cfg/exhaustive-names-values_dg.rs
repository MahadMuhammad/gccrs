// Check warning for unexpected cfg in the code.
//
//@ check-pass
//@ no-auto-check-cfg
//@ revisions: empty_cfg feature full
//@ [empty_cfg]compile-flags: --check-cfg=cfg()
//@ [feature]compile-flags: --check-cfg=cfg(feature,values("std"))
//@ [full]compile-flags: --check-cfg=cfg(feature,values("std")) --check-cfg=cfg()

#[cfg(unknown_key = "value")]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn f() {}

#[cfg(test = "value")]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn f() {}

#[cfg(feature = "unk")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
pub fn feat() {}

#[cfg(feature = "std")]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn feat() {}

#[cfg(windows)]
pub fn win() {}

#[cfg(unix)]
pub fn unix() {}

fn main() {}

