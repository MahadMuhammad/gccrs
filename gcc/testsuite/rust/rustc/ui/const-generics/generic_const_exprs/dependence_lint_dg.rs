//@ revisions: full gce
//@ compile-flags: -Zdeduplicate-diagnostics=yes

#![cfg_attr(gce, feature(generic_const_exprs))]
#![allow(incomplete_features)]

use std::mem::size_of;

fn foo<T>() {
    [0; size_of::<*mut T>()]; // lint on stable, error with `generic_const_exprs`
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
    let _: [u8; size_of::<*mut T>()]; // error on stable, error with gce
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    [0; if false { size_of::<T>() } else { 3 }]; // lint on stable, error with gce
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
    let _: [u8; if true { size_of::<T>() } else { 3 }]; // error on stable, error with gce
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

