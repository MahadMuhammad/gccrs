// Checks that const expressions have a useful note explaining why they can't be evaluated.
// The note should relate to the fact that it cannot be shown forall N that it maps 1-1 to a new
// type.

//@ revisions: full min
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

struct Collatz<const N: Option<usize>>;
// { dg-error "" "" { target *-*-* } .-1 }

impl <const N: usize> Collatz<{Some(N)}> {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }

struct Foo;

impl<const N: usize> Foo {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

