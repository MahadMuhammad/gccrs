#![feature(negative_impls)]
#![crate_type = "lib"]

impl !Copy for str {}
// { dg-error ".E0117." "" { target *-*-* } .-1 }

impl !Copy for fn() {}
// { dg-error ".E0117." "" { target *-*-* } .-1 }

impl !Copy for () {}
// { dg-error ".E0117." "" { target *-*-* } .-1 }

