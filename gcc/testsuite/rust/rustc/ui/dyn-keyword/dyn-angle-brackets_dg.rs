// See https://github.com/rust-lang/rust/issues/88508
//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
#![deny(bare_trait_objects)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;

#[derive(Debug)]
pub struct Foo;

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <fmt::Debug>::fmt(self, f)
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    }
}

fn main() {}

