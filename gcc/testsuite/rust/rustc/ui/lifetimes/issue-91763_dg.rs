//@ aux-build:issue-91763-aux.rs

#![deny(elided_lifetimes_in_paths)]

extern crate issue_91763_aux;

#[issue_91763_aux::repro]
fn f() -> Ptr<Thing>;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

