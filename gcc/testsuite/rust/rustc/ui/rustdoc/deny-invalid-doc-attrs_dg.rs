#![deny(invalid_doc_attributes)]
// { dg-note "" "" { target *-*-* } .-1 }
#![doc(x)]
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {}

