//@ run-rustfix
//@ check-pass
//@ compile-flags: --edition=2021
#![allow(incomplete_features)]
#![feature(expr_fragment_specifier_2024)]
#![warn(edition_2024_expr_fragment_specifier)]

macro_rules! m {
    ($e:expr) => { // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
        $e
    };
    ($($i:expr)*) => { }; // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
}

macro_rules! test {
    (expr) => {}
}

fn main() {
    m!(());
    test!(expr);
}

