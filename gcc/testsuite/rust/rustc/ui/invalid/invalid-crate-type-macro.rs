#![crate_type = foo!()] // { dg-error "" "" { target *-*-* } }

macro_rules! foo {
    () => {"rlib"};
}

fn main() {}

