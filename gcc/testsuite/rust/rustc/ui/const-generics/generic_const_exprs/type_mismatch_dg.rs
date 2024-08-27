#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Q {
    const ASSOC: usize;
}

impl<const N: u64> Q for [u8; N] {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }

pub fn q_user() -> [u8; <[u8; 13] as Q>::ASSOC] {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

pub fn main() {}

