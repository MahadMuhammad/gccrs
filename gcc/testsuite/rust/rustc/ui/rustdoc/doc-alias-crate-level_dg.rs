//@ compile-flags: -Zdeduplicate-diagnostics=no

#![crate_type = "lib"]

#![doc(alias = "not working!")] // { dg-error "" "" { target *-*-* } }

#[doc(alias = "shouldn't work!")] // { dg-error "" "" { target *-*-* } }
pub struct Foo;

