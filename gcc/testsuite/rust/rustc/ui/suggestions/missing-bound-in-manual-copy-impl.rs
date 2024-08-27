//@ run-rustfix
#![allow(dead_code)]

#[derive(Clone)]
struct Wrapper<T>(T);

impl<S> Copy for Wrapper<S> {}
// { dg-error ".E0204." "" { target *-*-* } .-1 }

fn main() {}

