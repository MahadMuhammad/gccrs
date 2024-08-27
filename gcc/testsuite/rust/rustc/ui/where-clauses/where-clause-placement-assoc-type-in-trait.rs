//@ check-pass
//@ run-rustfix

#![allow(dead_code)]
#![feature(associated_type_defaults)]

trait Trait {
    // Not fine, suggests moving.
    type Assoc where u32: Copy = ();
// { dg-warning "" "" { target *-*-* } .-1 }
    // Not fine, suggests moving `u32: Copy`
    type Assoc2 where u32: Copy = () where i32: Copy;
// { dg-warning "" "" { target *-*-* } .-1 }
}

fn main() {}

