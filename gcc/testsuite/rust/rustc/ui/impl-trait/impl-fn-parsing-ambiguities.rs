#![feature(impl_trait_in_fn_trait_return)]
use std::fmt::Debug;

fn a() -> impl Fn(&u8) -> impl Debug + '_ {
// { dg-error ".E0657." "" { target *-*-* } .-1 }
// { dg-error ".E0657." "" { target *-*-* } .-2 }
    |x| x
}

fn b() -> impl Fn() -> impl Debug + Send {
// { dg-error "" "" { target *-*-* } .-1 }
    || ()
}

fn main() {}

