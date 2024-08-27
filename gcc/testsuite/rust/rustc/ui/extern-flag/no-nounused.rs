//@ aux-crate:somedep=somedep.rs
//@ compile-flags: -Zunstable-options -Dunused-crate-dependencies
// { dg-additional-options "-frust-edition=2018" }

fn main() { // { dg-error "" "" { target *-*-* } }
}

