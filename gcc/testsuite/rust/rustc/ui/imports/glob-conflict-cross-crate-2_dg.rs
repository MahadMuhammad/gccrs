//@ aux-build:glob-conflict-cross-crate-2-extern.rs

extern crate glob_conflict_cross_crate_2_extern;

use glob_conflict_cross_crate_2_extern::*;

fn main() {
    let _a: C = 1; // { dg-error ".E0412." "" { target *-*-* } }
    //^ FIXME: `C` should be identified as an ambiguous item.
}

