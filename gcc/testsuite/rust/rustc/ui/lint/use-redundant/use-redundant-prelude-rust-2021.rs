//@ check-pass
// { dg-additional-options "-frust-edition=2021" }
#![warn(redundant_imports)]

use std::convert::TryFrom;// { dg-warning "" "" { target *-*-* } }
use std::convert::TryInto;// { dg-warning "" "" { target *-*-* } }

fn main() {
    let _e: Result<i32, _> = 8u8.try_into();
    let _f: Result<i32, _> = i32::try_from(8u8);
}

