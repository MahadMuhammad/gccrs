//@ aux-build:xcrate.rs
//@ compile-flags:--extern xcrate
// { dg-additional-options "-frust-edition=2018" }

use crate; // { dg-error "" "" { target *-*-* } }
use *; // { dg-error "" "" { target *-*-* } }

fn main() {
    let s = ::xcrate; // { dg-error ".E0423." "" { target *-*-* } }
// { dg-note ".E0423." "" { target *-*-* } .-1 }
}

