//! Check that pattern types don't implement traits of their base automatically

#![feature(pattern_types)]
#![feature(core_pattern_types)]
#![feature(core_pattern_type)]

use std::pat::pattern_type;

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
struct Nanoseconds(NanoI32);
// { dg-error ".E0369." "" { target *-*-* } .-1 }

type NanoI32 = crate::pattern_type!(i32 is 0..=999_999_999);

fn main() {
    let x = Nanoseconds(unsafe { std::mem::transmute(42) });
    let y = x.clone();
    if y == x {}
}

