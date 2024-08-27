#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn foo() -> [(); {
    let a: &'a ();
// { dg-error ".E0261." "" { target *-*-* } .-1 }
    10_usize
}] {
    loop {}
}

fn main() {}

