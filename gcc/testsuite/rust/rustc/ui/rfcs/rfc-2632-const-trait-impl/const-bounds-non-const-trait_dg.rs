// Regression test for issue #117244.
#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

trait NonConst {}

const fn perform<T: ~const NonConst>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn operate<T: const NonConst>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

