//@ run-rustfix

#![allow(dead_code)]

extern "C" {
    unsafe fn foo(); // { dg-error "" "" { target *-*-* } }
}

fn main() {}

