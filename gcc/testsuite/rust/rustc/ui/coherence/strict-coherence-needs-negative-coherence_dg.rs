#![feature(rustc_attrs)]

#[rustc_strict_coherence]
trait Foo {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

