//@ run-rustfix

pub T(#[allow(dead_code)] String);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

