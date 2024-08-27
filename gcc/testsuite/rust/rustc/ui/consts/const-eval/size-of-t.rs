// https://github.com/rust-lang/rust/issues/69228
// Used to give bogus suggestion about T not being Sized.

use std::mem::size_of;

fn foo<T>() {
    let _arr: [u8; size_of::<T>()];
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
}

fn main() {}

