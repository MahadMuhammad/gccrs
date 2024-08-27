#![feature(min_specialization)]

// An impl that has an erroneous const substitution should not specialize one
// that is well-formed.
#[derive(Clone)]
struct S<const L: usize>;

impl<const N: i32> Copy for S<N> {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
impl<const M: usize> Copy for S<M> {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

