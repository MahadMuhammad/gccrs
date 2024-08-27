// Regression test for the ICE described in #86820.

#![allow(unused, dead_code)]
use std::ops::BitAnd;

const C: fn() = || is_set();
fn is_set() {
    0xffu8.bit::<0>();
}

trait Bits {
    fn bit<const I: u8>(self) -> bool;
}

impl Bits for u8 {
    fn bit<const I: usize>(self) -> bool {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
        let i = 1 << I;
        let mask = u8::from(i);
        mask & self == mask
    }
}

fn main() {}

