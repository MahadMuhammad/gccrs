//@ compile-flags: -Z span-debug
//@ error-pattern:custom inner attributes are unstable
//@ error-pattern:inner macro attributes are unstable
//@ error-pattern:this was previously accepted
//@ aux-build:test-macros.rs

#![no_std] // Don't load unnecessary hygiene information from std
extern crate std;

#[macro_use]
extern crate test_macros;

#[deny(unused_attributes)]
mod module_with_attrs;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

fn main() {}

