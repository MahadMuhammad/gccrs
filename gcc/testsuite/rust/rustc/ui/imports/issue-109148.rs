// { dg-additional-options "-frust-edition= 2021" }

// https://github.com/rust-lang/rust/pull/111761#issuecomment-1557777314
macro_rules! m {
    () => {
        extern crate core as std;
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

m!();

use std::mem;

fn main() {}

