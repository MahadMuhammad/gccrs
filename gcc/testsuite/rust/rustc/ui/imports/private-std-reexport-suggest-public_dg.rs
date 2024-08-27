//@ run-rustfix
#![allow(unused_imports)]
fn main() {
    use foo::mem; // { dg-error ".E0603." "" { target *-*-* } }
}

pub mod foo {
    use std::mem;
}

