//@ aux-build:const-stability-attribute-implies.rs
#![crate_type = "lib"]
#![deny(stable_features)]
#![feature(const_foo)]
// { dg-error "" "" { target *-*-* } .-1 }

// Tests that the use of `implied_by` in the `#[rustc_const_unstable]` attribute results in a
// diagnostic mentioning partial stabilization, and that given the implied unstable feature is
// unused (there is no `foobar` call), that the compiler suggests removing the flag.

extern crate const_stability_attribute_implies;
use const_stability_attribute_implies::foo;

pub const fn bar() -> u32 {
    foo();
    0
}

pub const VAR: u32 = bar();

