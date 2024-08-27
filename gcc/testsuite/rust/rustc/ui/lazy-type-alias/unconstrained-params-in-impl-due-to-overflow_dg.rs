#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

impl<T> Loop<T> {} // { dg-error ".E0207." "" { target *-*-* } }

type Loop<T> = Loop<T>; // { dg-error ".E0275." "" { target *-*-* } }

fn main() {}

