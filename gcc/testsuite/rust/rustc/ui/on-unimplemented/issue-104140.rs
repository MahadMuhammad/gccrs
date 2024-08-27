#![feature(rustc_attrs)]

trait Foo {}

#[rustc_on_unimplemented] // { dg-error "" "" { target *-*-* } }
impl Foo for u32 {}

fn main() {}

