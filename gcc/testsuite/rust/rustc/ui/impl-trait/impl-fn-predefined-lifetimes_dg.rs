#![feature(impl_trait_in_fn_trait_return)]
use std::fmt::Debug;

fn a<'a>() -> impl Fn(&'a u8) -> (impl Debug + '_) {
// { dg-error ".E0720." "" { target *-*-* } .-1 }
    |x| x
// { dg-error ".E0792." "" { target *-*-* } .-1 }
}

fn _b<'a>() -> impl Fn(&'a u8) -> (impl Debug + 'a) {
    a()
}

fn main() {}

