#![feature(fn_traits, unboxed_closures, tuple_trait)]

use std::default::Default;
use std::marker::Tuple;

fn wrap<P: Tuple + Default, T>(func: impl Fn<P, Output = T>) {
    let x: P = Default::default();
    // Should be: `func.call(x);`
    func(x);
// { dg-error ".E0059." "" { target *-*-* } .-1 }
}

fn foo() {}

fn main() {
    wrap(foo);
}

