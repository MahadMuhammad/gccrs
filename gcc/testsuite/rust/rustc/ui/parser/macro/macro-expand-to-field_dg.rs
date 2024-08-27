//@ compile-flags: --crate-type=lib

// https://github.com/rust-lang/rust/issues/113766

macro_rules! field {
    ($name:ident:$type:ty) => {
        $name:$type
    };
}

macro_rules! variant {
    ($name:ident) => {
        $name
    }
}

struct Struct {
// { dg-note "" "" { target *-*-* } .-1 }
    field!(bar:u128),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    a: u32,
    b: u32,
    field!(recovers:()),
}

enum EnumVariant {
    variant!(whoops),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    U32,
    F64,
    variant!(recovers),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    Data { // { dg-note "" "" { target *-*-* } }
        field!(x:u32),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    }
}

enum EnumVariantField {
    Named { // { dg-note "" "" { target *-*-* } }
        field!(oopsies:()),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
        field!(oopsies2:()),
    },
}

union Union {
// { dg-note "" "" { target *-*-* } .-1 }
    A: u32,
    field!(oopsies:()),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    B: u32,
    field!(recovers:()),
}

// https://github.com/rust-lang/rust/issues/114636

#[derive(Debug)]
pub struct Lazy {
// { dg-note "" "" { target *-*-* } .-1 }
    unreachable!()
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
}

fn main() {}

