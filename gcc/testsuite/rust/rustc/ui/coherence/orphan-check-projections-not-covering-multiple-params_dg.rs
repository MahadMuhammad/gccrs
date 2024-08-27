//@ revisions: classic next
//@[next] compile-flags: -Znext-solver

//@ check-pass
//@ compile-flags: --crate-type=lib
//@ aux-crate:foreign=parametrized-trait.rs
// { dg-additional-options "-frust-edition=2021" }

trait Trait<T, U> { type Assoc; }

impl<T, U> Trait<T, U> for () {
    type Assoc = LocalTy;
}

struct LocalTy;

impl<T, U> foreign::Trait0<LocalTy, T, U> for <() as Trait<T, U>>::Assoc {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }


fn main() {}

