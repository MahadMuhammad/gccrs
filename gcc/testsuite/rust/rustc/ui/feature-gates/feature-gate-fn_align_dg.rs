#![crate_type = "lib"]

#[repr(align(16))] // { dg-error ".E0658." "" { target *-*-* } }
fn requires_alignment() {}

