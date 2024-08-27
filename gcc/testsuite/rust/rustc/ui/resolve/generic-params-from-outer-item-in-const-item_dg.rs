// Regression test for issue #115720.
// If a const item contains generic params from an outer items, only suggest
// turning the const item generic if the feature `generic_const_items` is enabled.

//@ revisions: default generic_const_items

#![cfg_attr(generic_const_items, feature(generic_const_items))]
#![feature(generic_const_exprs)] // only used for the test case "outer struct"
#![allow(incomplete_features)]

fn outer<T: Tr>() { // outer function
    const K: u32 = T::C;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

impl<T> Tr for T { // outer impl block
    const C: u32 = {
        const I: u32 = T::C;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        I
    };
}

struct S<T: Tr>(U32<{ // outer struct
    const _: u32 = T::C;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    0
}>);

trait Tr {
    const C: u32;
}

struct U32<const N: u32>;

fn main() {}

