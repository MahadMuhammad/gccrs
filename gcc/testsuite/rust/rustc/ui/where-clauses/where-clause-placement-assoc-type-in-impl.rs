//@ check-pass
//@ run-rustfix

#![allow(dead_code)]

trait Trait {
    // Fine.
    type Assoc where u32: Copy;
    // Fine.
    type Assoc2 where u32: Copy, i32: Copy;
    //
    type Assoc3;
}

impl Trait for u32 {
    // Not fine, suggests moving.
    type Assoc where u32: Copy = ();
// { dg-warning "" "" { target *-*-* } .-1 }
    // Not fine, suggests moving `u32: Copy`
    type Assoc2 where u32: Copy = () where i32: Copy;
// { dg-warning "" "" { target *-*-* } .-1 }
    type Assoc3 where = ();
// { dg-warning "" "" { target *-*-* } .-1 }
}

impl Trait for i32 {
    // Fine.
    type Assoc = () where u32: Copy;
    // Not fine, suggests moving both.
    type Assoc2 where u32: Copy, i32: Copy = ();
// { dg-warning "" "" { target *-*-* } .-1 }
    type Assoc3 where = () where;
// { dg-warning "" "" { target *-*-* } .-1 }
}

fn main() {}

