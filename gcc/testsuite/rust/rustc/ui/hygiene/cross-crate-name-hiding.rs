// Check that an item defined by a 2.0 macro in another crate cannot be used in
// another crate.

//@ aux-build:pub_hygiene.rs

extern crate pub_hygiene;

use pub_hygiene::*;

fn main() {
    let x = MyStruct {};
// { dg-error ".E0422." "" { target *-*-* } .-1 }
}

