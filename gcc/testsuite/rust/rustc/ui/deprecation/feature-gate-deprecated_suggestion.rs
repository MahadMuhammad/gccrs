//@ compile-flags: --crate-type=lib

#![no_implicit_prelude]

#[deprecated(suggestion = "foo")] // { dg-error "" "" { target *-*-* } }
struct Foo {}

