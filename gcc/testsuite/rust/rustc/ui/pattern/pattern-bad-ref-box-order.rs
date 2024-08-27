//@ run-rustfix

#![feature(box_patterns)]
#![allow(dead_code)]

fn foo(f: Option<Box<i32>>) {
    match f {
        Some(ref box _i) => {},
// { dg-error "" "" { target *-*-* } .-1 }
        None => {}
    }
}

fn main() { }

