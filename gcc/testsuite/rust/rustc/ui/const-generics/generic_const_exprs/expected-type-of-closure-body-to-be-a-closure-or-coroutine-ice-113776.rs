// issue: rust-lang/rust#113776
// ice: expected type of closure body to be a closure or coroutine
// { dg-additional-options "-frust-edition= 2021" }
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::ops::SubAssign;

fn f<T>(
    data: &[(); {
         let f: F = async { 1 };
// { dg-error ".E0412." "" { target *-*-* } .-1 }

         1
     }],
) -> impl Iterator<Item = SubAssign> {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
// { dg-error ".E0782." "" { target *-*-* } .-3 }
// { dg-error ".E0782." "" { target *-*-* } .-4 }
}

pub fn main() {}

