// no need to create a new aux file, we can use an existing.
//@ aux-build: crateresolve1-1.rs

// set same metadata as `crateresolve1`
#![crate_name = "crateresolve1"]
#![crate_type = "lib"]

extern crate crateresolve1; // { dg-error ".E0519." "" { target *-*-* } }

