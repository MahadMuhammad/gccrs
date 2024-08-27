//@ run-rustfix
//@ aux-build:issue-69725.rs
#![allow(dead_code)]

extern crate issue_69725;
use issue_69725::Struct;

fn crash<A>() {
    let _ = Struct::<A>::new().clone();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {}

