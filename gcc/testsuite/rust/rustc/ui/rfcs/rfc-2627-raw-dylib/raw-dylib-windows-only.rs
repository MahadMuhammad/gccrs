//@ ignore-windows
//@ compile-flags: --crate-type lib
#[link(name = "foo", kind = "raw-dylib")]
// { dg-error ".E0455." "" { target *-*-* } .-1 }
extern "C" {}

