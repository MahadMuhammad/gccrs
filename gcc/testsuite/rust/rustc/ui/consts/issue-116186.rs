#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn something(path: [usize; N]) -> impl Clone {
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    match path {
        [] => 0, // { dg-error ".E0730." "" { target *-*-* } }
        _ => 1,
    };
}

fn main() {}

