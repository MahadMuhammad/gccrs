//@ aux-crate:noprelude:somedep=somedep.rs
//@ compile-flags: -Zunstable-options
// { dg-additional-options "-frust-edition=2018" }

fn main() {
    somedep::somefun();  // { dg-error ".E0433." "" { target *-*-* } }
}

