#![feature(impl_trait_in_fn_trait_return)]
use std::fmt::Debug;

fn a() -> impl Fn(&u8) -> (impl Debug + '_) {
// { dg-error ".E0657." "" { target *-*-* } .-1 }
    |x| x
}

fn b() -> impl for<'a> Fn(&'a u8) -> (impl Debug + 'a) {
// { dg-error ".E0657." "" { target *-*-* } .-1 }
    |x| x
}

fn c() -> impl for<'a> Fn(&'a u8) -> (impl Debug + '_) {
// { dg-error ".E0657." "" { target *-*-* } .-1 }
    |x| x
}

fn d() -> impl Fn() -> (impl Debug + '_) {
// { dg-error ".E0106." "" { target *-*-* } .-1 }
    || ()
}

fn main() {}

