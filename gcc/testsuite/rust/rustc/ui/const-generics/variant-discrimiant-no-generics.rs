//@ revisions: full min

#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

enum Foo<const N: isize> {
    Variant = N,
// { dg-error "" "" { target *-*-* } .-1 }
}

enum Owo<const N: isize> {
    Variant = { N + 1 },
// { dg-error "" "" { target *-*-* } .-1 }
}

#[repr(isize)]
enum Bar<T> {
    Variant = { std::mem::size_of::<T>() as isize },
// { dg-error "" "" { target *-*-* } .-1 }
}

#[repr(isize)]
enum UwU<'a> {
    Variant = {
        let a: &'a ();
// { dg-error "" "" { target *-*-* } .-1 }
        10_isize
    },
    Other(&'a ()),
}

fn main() {}

