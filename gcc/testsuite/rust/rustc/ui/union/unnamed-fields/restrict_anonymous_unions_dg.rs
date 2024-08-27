#![allow(incomplete_features)]
#![feature(unnamed_fields)]

struct F {
    field1: union { field2: u8 }, // { dg-error "" "" { target *-*-* } }
    _: union { field3: u8 },
}

struct G {
    _: (u8, u8), // { dg-error "" "" { target *-*-* } }
}

union H {
    field1: union { field2: u8 }, // { dg-error "" "" { target *-*-* } }
    _: union { field3: u8 },
}

union I {
    _: (u8, u8), // { dg-error "" "" { target *-*-* } }
}

enum K {
    M {
        _ : union { field: u8 }, // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    },
    N {
        _ : u8, // { dg-error "" "" { target *-*-* } }
    }
}

fn main() {}

