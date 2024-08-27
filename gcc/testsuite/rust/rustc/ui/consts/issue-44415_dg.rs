#![feature(core_intrinsics)]

use std::intrinsics;

struct Foo {
    bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
// { dg-error ".E0391." "" { target *-*-* } .-1 }
    x: usize,
}

fn main() {}

