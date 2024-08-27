#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct U;

impl U {
    // Don't imply any bounds here.

    type NoTyOutliv<'a, T> = &'a T; // { dg-error ".E0309." "" { target *-*-* } }
    type NoReOutliv<'a, 'b> = &'a &'b (); // { dg-error ".E0491." "" { target *-*-* } }
}

fn main() {}

