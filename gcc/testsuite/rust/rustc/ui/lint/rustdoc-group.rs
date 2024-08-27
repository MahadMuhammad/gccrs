//@ check-pass
//@ compile-flags: --crate-type lib
#![deny(rustdoc)]
// { dg-warning "" "" { target *-*-* } .-1 }
#![deny(rustdoc::all)] // has no effect when run with rustc directly

