// Ensure that we properly deal with missing/placeholder types inside GACs.
// issue: rust-lang/rust#124833
#![feature(generic_const_items)]
#![allow(incomplete_features)]

trait Trait {
    const K<T>: T;
    const Q<'a>: &'a str;
}

impl Trait for () {
    const K<T> = ();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
    const Q = "";
// { dg-error ".E0195." "" { target *-*-* } .-1 }
// { dg-error ".E0195." "" { target *-*-* } .-2 }
}

fn main() {}

