#![crate_type = "lib"]
#![deny(invalid_doc_attributes)]

#![doc(test)]
// { dg-error "" "" { target *-*-* } .-1 }
#![doc(test = "hello")]
// { dg-error "" "" { target *-*-* } .-1 }
#![doc(test(a))]
// { dg-error "" "" { target *-*-* } .-1 }

pub fn foo() {}

