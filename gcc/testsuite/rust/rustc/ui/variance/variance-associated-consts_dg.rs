// Test that the variance computation considers types that
// appear in const expressions to be invariant.

#![feature(rustc_attrs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait Trait {
    const Const: usize;
}

#[rustc_variance]
struct Foo<T: Trait> { // { dg-error "" "" { target *-*-* } }
    field: [u8; <T as Trait>::Const]
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() { }

