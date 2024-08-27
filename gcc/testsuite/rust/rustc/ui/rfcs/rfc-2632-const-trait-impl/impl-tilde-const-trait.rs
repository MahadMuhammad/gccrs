#![feature(const_trait_impl)]

struct S;
trait T {}

impl ~const T for S {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

