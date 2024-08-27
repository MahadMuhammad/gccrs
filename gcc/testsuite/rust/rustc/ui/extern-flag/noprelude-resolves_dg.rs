//@ check-pass
//@ aux-crate:noprelude:somedep=somedep.rs
//@ compile-flags: -Zunstable-options
// { dg-additional-options "-frust-edition=2018" }

// `extern crate` can be used to add to prelude.
extern crate somedep;

fn main() {
    somedep::somefun();
}

