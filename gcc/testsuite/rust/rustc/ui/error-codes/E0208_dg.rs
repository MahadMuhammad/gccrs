#![feature(rustc_attrs)]

#[rustc_variance]
struct Foo<'a, T> { // { dg-error "" "" { target *-*-* } }
    t: &'a mut T,
}

fn main() {}

