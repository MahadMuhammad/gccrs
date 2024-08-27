#![feature(type_ascription)]

enum Bug<S> { // { dg-error ".E0392." "" { target *-*-* } }
    Var = type_ascribe!(0, S),
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

