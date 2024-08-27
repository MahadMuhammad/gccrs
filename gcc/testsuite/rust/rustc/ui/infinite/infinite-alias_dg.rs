//@ aux-build: alias.rs
// regression test for 108160

extern crate alias;

use alias::Wrapper;
struct Rec(Wrapper<Rec>); // { dg-error ".E0072." "" { target *-*-* } }

fn main() {}

