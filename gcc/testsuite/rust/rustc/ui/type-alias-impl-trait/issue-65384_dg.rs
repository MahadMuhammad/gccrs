#![feature(type_alias_impl_trait)]

trait MyTrait {}

impl MyTrait for () {}

type Bar = impl MyTrait;

impl MyTrait for Bar {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn bazr() -> Bar { }

fn main() {}

