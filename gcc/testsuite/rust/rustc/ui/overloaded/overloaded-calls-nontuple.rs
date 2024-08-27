#![feature(fn_traits, unboxed_closures)]

use std::ops::FnMut;

struct S {
    x: isize,
    y: isize,
}

impl FnMut<isize> for S {
// { dg-error ".E0059." "" { target *-*-* } .-1 }
    extern "rust-call" fn call_mut(&mut self, z: isize) -> isize {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        self.x + self.y + z
    }
}

impl FnOnce<isize> for S {
// { dg-error ".E0059." "" { target *-*-* } .-1 }
    type Output = isize;
    extern "rust-call" fn call_once(mut self, z: isize) -> isize {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        self.call_mut(z) // { dg-error ".E0277." "" { target *-*-* } }
    }
}

fn main() {
    let mut s = S { x: 1, y: 2 };
    drop(s(3)) // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

