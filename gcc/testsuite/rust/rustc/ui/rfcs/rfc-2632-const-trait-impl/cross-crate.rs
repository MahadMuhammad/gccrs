//@ revisions: stock gated stocknc gatednc
//@ [gated] check-pass
//@ compile-flags: -Znext-solver
#![cfg_attr(any(gated, gatednc), feature(const_trait_impl, effects))]
#![allow(incomplete_features)]

//@ aux-build: cross-crate.rs
extern crate cross_crate;

use cross_crate::*;

fn non_const_context() {
    NonConst.func();
    Const.func();
}

const fn const_context() {
    #[cfg(any(stocknc, gatednc))]
    NonConst.func();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    Const.func();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

