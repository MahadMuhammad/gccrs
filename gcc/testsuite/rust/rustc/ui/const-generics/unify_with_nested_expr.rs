#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn foo<const N: usize>()
where
    [(); N + 1 + 1]:,
{
    bar();
// { dg-error ".E0284." "" { target *-*-* } .-1 }
}

fn bar<const N: usize>()
where
    [(); N + 1]:,
{
}

fn main() {}

