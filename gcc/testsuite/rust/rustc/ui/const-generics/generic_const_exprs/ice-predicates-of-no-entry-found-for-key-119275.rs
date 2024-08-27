// test for ICE #119275 "no entry found for key" in predicates_of.rs

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn bug<const N: Nat>(&self)
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
where
    for<const N: usize = 3, T = u32> [(); COT::BYTES]:,
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }
// { dg-error ".E0433." "" { target *-*-* } .-3 }
// { dg-error ".E0433." "" { target *-*-* } .-4 }
// { dg-error ".E0433." "" { target *-*-* } .-5 }
{
}

pub fn main() {}

