//@check-pass
//@revisions: old next
//@[next] compile-flags: -Znext-solver

#![feature(ptr_metadata)]
#![feature(dyn_star)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::fmt::Debug;
use std::ptr::Thin;

fn check_thin<T: ?Sized + Thin>() {}

fn main() {
    check_thin::<dyn* Debug>();
}

