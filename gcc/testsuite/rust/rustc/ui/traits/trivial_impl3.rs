//! Check that we don't break orphan rules.
//! The dependency may add an impl for `u8` later,
//! which would break this crate. We want to avoid adding
//! more ways in which adding an impl can be a breaking change.

//@ aux-build:trivial3.rs

extern crate trivial3;

pub trait Foo {
    fn foo()
    where
        Self: trivial3::Trait;
}

impl Foo for u8 {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }

fn main() {}

