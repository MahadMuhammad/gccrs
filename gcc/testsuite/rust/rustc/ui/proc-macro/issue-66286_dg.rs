//@ aux-build:issue-66286.rs

// Regression test for #66286.

extern crate issue_66286;

#[issue_66286::vec_ice]
pub extern fn foo(_: Vec(u32)) -> u32 {
// { dg-error ".E0214." "" { target *-*-* } .-1 }
    0
}

fn main() {}

