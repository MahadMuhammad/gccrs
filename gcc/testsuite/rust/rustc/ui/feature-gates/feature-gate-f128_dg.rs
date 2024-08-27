//@ revisions: e2015 e2018
//
// { dg-additional-options "-frust-edition=2018" }

#![allow(unused)]

const A: f128 = 10.0; // { dg-error "" "" { target *-*-* } }

pub fn main() {
    let a: f128 = 100.0; // { dg-error "" "" { target *-*-* } }
    let b = 0.0f128; // { dg-error "" "" { target *-*-* } }
    foo(1.23);
}

fn foo(a: f128) {} // { dg-error "" "" { target *-*-* } }

struct Bar {
    a: f128, // { dg-error "" "" { target *-*-* } }
}

