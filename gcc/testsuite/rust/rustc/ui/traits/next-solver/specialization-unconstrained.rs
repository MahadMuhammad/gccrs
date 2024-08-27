//@ compile-flags: -Znext-solver

#![feature(specialization)]
// { dg-warning "" "" { target *-*-* } .-1 }

// Do not treat the RHS of a projection-goal as an unconstrained `Certainty::Yes` response
// if the impl is still further specializable.

trait Default {
   type Id;
}

impl<T> Default for T {
   default type Id = T;
}

fn test<T: Default<Id = U>, U>() {}

fn main() {
    test::<u32, ()>();
// { dg-error ".E0284." "" { target *-*-* } .-1 }
}

