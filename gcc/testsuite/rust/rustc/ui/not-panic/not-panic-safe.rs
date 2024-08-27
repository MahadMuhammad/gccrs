#![allow(dead_code)]

use std::panic::UnwindSafe;

fn assert<T: UnwindSafe + ?Sized>() {}

fn main() {
    assert::<&mut &mut &i32>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

