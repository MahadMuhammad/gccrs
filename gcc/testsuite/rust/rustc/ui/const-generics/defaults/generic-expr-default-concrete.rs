#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Foo<const N: usize, const M: usize = { N + 1 }>;
fn no_constraining() -> Foo<10> {
    Foo::<10, 11>
}

pub fn different_than_default() -> Foo<10> {
    Foo::<10, 12>
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

