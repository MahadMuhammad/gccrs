//@ aux-build:const-stability-attribute-implies.rs
#![crate_type = "lib"]
#![deny(stable_features)]
#![feature(const_foo)]
// { dg-error "" "" { target *-*-* } .-1 }

// Tests that the use of `implied_by` in the `#[rustc_const_unstable]` attribute results in a
// diagnostic mentioning partial stabilization and that given the implied unstable feature is
// used (there is a `const_foobar` call), that the compiler suggests changing to that feature and
// doesn't error about its use.

extern crate const_stability_attribute_implies;
use const_stability_attribute_implies::{foo, foobar};

pub const fn bar() -> u32 {
    foo();
    foobar(); // no error!
    0
}

pub const VAR: u32 = bar();

