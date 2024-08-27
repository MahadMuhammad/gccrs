// Opaque types never cover type parameters.

//@ revisions: classic next
//@[next] compile-flags: -Znext-solver

//@ aux-crate:foreign=parametrized-trait.rs
// { dg-additional-options "-frust-edition=2021" }

#![feature(type_alias_impl_trait)]

type Identity<T> = impl Sized;

fn define_identity<T>(x: T) -> Identity<T> {
    x
}

impl<T> foreign::Trait0<Local, T, ()> for Identity<T> {}
// { dg-error "" "" { target *-*-* } .-1 }

type Opaque<T> = impl Sized;

fn define_local<T>() -> Opaque<T> {
    Local
}

impl<T> foreign::Trait1<Local, T> for Opaque<T> {}
// { dg-error "" "" { target *-*-* } .-1 }

struct Local;

fn main() {}

