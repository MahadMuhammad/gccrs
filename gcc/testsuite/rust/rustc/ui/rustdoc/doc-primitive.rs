#![deny(invalid_doc_attributes)]

#[doc(primitive = "foo")]
// { dg-error "" "" { target *-*-* } .-1 }
mod bar {}

fn main() {}

