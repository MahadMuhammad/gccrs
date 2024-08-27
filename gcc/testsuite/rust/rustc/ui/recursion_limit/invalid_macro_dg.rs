#![recursion_limit = foo!()] // { dg-error "" "" { target *-*-* } }

macro_rules! foo {
    () => {"128"};
}

fn main() {}

