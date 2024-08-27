#![feature(pattern_types)]
#![feature(core_pattern_types)]
#![feature(core_pattern_type)]

use std::pat::pattern_type;

type NonNullU32_2 = pattern_type!(u32 is 1..=);
// { dg-error ".E0586." "" { target *-*-* } .-1 }
type Positive2 = pattern_type!(i32 is 0..=);
// { dg-error ".E0586." "" { target *-*-* } .-1 }
type Wild = pattern_type!(() is _);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

