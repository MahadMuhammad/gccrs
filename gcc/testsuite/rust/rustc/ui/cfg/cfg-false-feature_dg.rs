// Features above `cfg(FALSE)` are in effect in a fully unconfigured crate (issue #104633).

//@ check-pass
//@ compile-flags: --crate-type lib

#![feature(decl_macro)]
#![cfg(FALSE)]
#![feature(box_syntax)]

macro mac() {} // OK

trait A = Clone; // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {
    let box _ = Box::new(0); // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
}

