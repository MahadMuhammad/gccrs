//@ run-rustfix
#![allow(dead_code)]

fn mutate(_y: &mut i32) {}

fn foo(&x: &i32) {
    mutate(&mut x);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
}

fn main() {}

