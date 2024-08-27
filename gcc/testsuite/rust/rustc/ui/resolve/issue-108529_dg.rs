#![allow(nonstandard_style)]
use f::f::f; // { dg-error ".E0432." "" { target *-*-* } }

trait f {
    extern "C" fn f();
}

fn main() {}

