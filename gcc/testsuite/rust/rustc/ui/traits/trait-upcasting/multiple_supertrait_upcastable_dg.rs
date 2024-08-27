#![feature(multiple_supertrait_upcastable)]
#![deny(multiple_supertrait_upcastable)]

trait A {}
trait B {}

trait C: A + B {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

