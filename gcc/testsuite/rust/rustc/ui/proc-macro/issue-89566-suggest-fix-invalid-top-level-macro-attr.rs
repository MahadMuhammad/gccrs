//@ run-rustfix

#![derive(Debug)] // { dg-error "" "" { target *-*-* } }
#[allow(dead_code)]
struct Test {}

fn main() {}

