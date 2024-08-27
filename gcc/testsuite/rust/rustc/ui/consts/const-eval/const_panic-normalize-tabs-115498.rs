#![crate_type = "lib"]

struct Bug([u8; panic!{"\t"}]);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-note ".E0080." "" { target *-*-* } .-2 }

