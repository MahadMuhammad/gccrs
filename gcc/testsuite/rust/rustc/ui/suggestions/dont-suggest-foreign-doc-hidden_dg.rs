//@ aux-build:hidden-struct.rs
//@ compile-flags: --crate-type lib

extern crate hidden_struct;

#[doc(hidden)]
mod local {
    pub struct Foo;
}

pub fn test(_: Foo) {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }

pub fn test2(_: Bar) {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }

