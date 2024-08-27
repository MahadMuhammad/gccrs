#![crate_type = "lib"]
#![feature(unnamed_fields)]
#![allow(unused, incomplete_features)]

enum K {
    M {
        _ : struct { field: u8 },
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

