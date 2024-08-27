// { dg-additional-options "-frust-edition= 2021" }
//@ aux-build: extern-with-ambiguous-1-extern.rs

// `extern-with-ambiguous-1-extern.rs` doesn't has
// ambiguous, just for compare.

extern crate extern_with_ambiguous_1_extern;

mod s {
    pub mod error {
        use extern_with_ambiguous_1_extern::*;
    }
}
use s::*;
use extern_with_ambiguous_1_extern::*;
use error::*;
// { dg-error ".E0659." "" { target *-*-* } .-1 }

fn main() {}

