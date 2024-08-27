#![allow(dead_code)]
fn foo() {}

#![feature(iter_array_chunks)] // { dg-error "" "" { target *-*-* } }
fn bar() {}

fn main() {
    foo();
    bar();
}

