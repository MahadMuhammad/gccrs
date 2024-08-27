#![deny(unused_attributes)]

#[doc(alias = "A")]
#[doc(alias = "A")] // { dg-error "" "" { target *-*-* } }
#[doc(alias = "B")]
#[doc(alias("B"))] // { dg-error "" "" { target *-*-* } }
pub struct Foo;

fn main() {}

