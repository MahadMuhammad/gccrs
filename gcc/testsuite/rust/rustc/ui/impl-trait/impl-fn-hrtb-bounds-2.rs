#![feature(impl_trait_in_fn_trait_return)]
use std::fmt::Debug;

fn a() -> impl Fn(&u8) -> impl Debug {
    |x| x // { dg-error ".E0700." "" { target *-*-* } }
}

fn main() {}

