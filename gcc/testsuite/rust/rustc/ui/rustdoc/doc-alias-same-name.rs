#![crate_type = "lib"]

#[doc(alias = "Foo")] // { dg-error "" "" { target *-*-* } }
pub struct Foo;

