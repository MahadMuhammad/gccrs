//@ run-rustfix

#![allow(warnings)]

fn no_restriction<T>(x: &()) -> &() {
    with_restriction::<T>(x) // { dg-error ".E0311." "" { target *-*-* } }
}

fn with_restriction<'a, T: 'a>(x: &'a ()) -> &'a () {
    x
}

fn main() {}

