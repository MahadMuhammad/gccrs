// Regression test of #43913.

//@ run-rustfix

#![feature(trait_alias)]
#![allow(bare_trait_objects, dead_code)]

type Strings = Iterator<Item=String>;

struct Struct<S: Strings>(S);
// { dg-error ".E0404." "" { target *-*-* } .-1 }

fn main() {}

