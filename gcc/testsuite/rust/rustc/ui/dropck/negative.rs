#![feature(negative_impls)]

struct NonDrop;
impl !Drop for NonDrop {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

