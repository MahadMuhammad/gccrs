#![feature(doc_keyword)] // { dg-error ".E0557." "" { target *-*-* } }
#![feature(doc_primitive)] // { dg-error ".E0557." "" { target *-*-* } }
#![crate_type = "lib"]

pub fn foo() {}

