//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(associated_type_defaults)]

use std::ops::Deref;

trait UnsafeCopy {
    type Copy<T>: Copy = Box<T>;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    fn copy<T>(x: &Self::Copy<T>) -> Self::Copy<T> {
        *x
    }
}

impl<T> UnsafeCopy for T {}

fn main() {
    let b = Box::new(42usize);
    let copy = <()>::copy(&b);

    let raw_b = Box::deref(&b) as *const _;
    let raw_copy = Box::deref(&copy) as *const _;

    // assert the addresses.
    assert_eq!(raw_b, raw_copy);
}

