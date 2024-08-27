#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }

// Regression test for #125770 which would ICE under the old effects desugaring that
// created a const generic parameter for constness on `Add`.

use std::ops::Add;

pub struct Dimension;

pub struct Quantity<S, const D: Dimension>(S);
// { dg-error "" "" { target *-*-* } .-1 }

impl<const D: Dimension, LHS, RHS> Add<LHS, D> for Quantity<LHS, { Dimension }> {}
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }

pub fn add<const U: Dimension>(x: Quantity<f32, U>) -> Quantity<f32, U> {
// { dg-error "" "" { target *-*-* } .-1 }
    x + y
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

fn main() {}

