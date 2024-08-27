#![deny(unreachable_code)]
#![allow(dead_code)]

use std::ptr;
pub unsafe fn g() {
    return;
    if *ptr::null() {}; // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
}

pub fn main() {}

