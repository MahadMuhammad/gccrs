#![allow(incomplete_features)]
#![feature(generic_const_exprs, const_trait_impl, effects)]

const fn with_positive<F: ~const Fn()>() {}
// { dg-error "" "" { target *-*-* } .-1 }

pub fn main() {}

