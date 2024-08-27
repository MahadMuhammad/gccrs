//@ revisions: e2015 e2018
//
// { dg-additional-options "-frust-edition=2018" }

#![allow(unused)]

const A: f16 = 10.0; // { dg-error "" "" { target *-*-* } }

pub fn main() {
    let a: f16 = 100.0; // { dg-error "" "" { target *-*-* } }
    let b = 0.0f16; // { dg-error "" "" { target *-*-* } }
    foo(1.23);
}

fn foo(a: f16) {} // { dg-error "" "" { target *-*-* } }

struct Bar {
    a: f16, // { dg-error "" "" { target *-*-* } }
}

