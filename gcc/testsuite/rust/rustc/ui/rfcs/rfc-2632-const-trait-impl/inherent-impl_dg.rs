#![feature(const_trait_impl)]
#![allow(bare_trait_objects)]

struct S;
trait T {}

impl const S {}
// { dg-error "" "" { target *-*-* } .-1 }

impl const T {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

