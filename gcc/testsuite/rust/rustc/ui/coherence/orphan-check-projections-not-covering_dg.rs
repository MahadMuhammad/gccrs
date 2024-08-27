// Regression test for issue #99554.
// Projections might not cover type parameters.

//@ revisions: classic next
//@[next] compile-flags: -Znext-solver

//@ check-pass
//@ compile-flags: --crate-type=lib
//@ aux-crate:foreign=parametrized-trait.rs
// { dg-additional-options "-frust-edition=2021" }

trait Identity {
    type Output;
}

impl<T> Identity for T {
    type Output = T;
}

struct Local;

impl<T> foreign::Trait0<Local, T, ()> for <T as Identity>::Output {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }


impl<T> foreign::Trait0<<T as Identity>::Output, Local, T> for Option<T> {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

pub trait Deferred {
    type Output;
}

// A downstream user could implement
//
//     impl<T> Deferred for Type<T> { type Output = T; }
//     struct Type<T>(T);
//
impl<T: Deferred> foreign::Trait1<Local, T> for <T as Deferred>::Output {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

