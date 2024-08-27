// Regression test for
// https://github.com/rust-lang/rust/issues/67945#issuecomment-572617285
// Make sure we don't emit an E0277 error.

//@ revisions: full min
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

struct Bug<S> { // { dg-error "" "" { target *-*-* } }
    A: [(); { // { dg-error "" "" { target *-*-* } }
        let x: Option<S> = None;
// { dg-error "" "" { target *-*-* } .-1 }
        0
    }],
}

fn main() {}

