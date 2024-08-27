#![feature(trait_alias, marker_trait_attr)]

#[marker]
// { dg-error "" "" { target *-*-* } .-1 }
trait Foo = Send;

fn main() {}

