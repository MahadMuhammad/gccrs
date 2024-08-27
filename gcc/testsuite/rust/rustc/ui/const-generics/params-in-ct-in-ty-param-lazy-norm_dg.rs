//@ revisions: full min
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

struct Foo<T, U = [u8; std::mem::size_of::<T>()]>(T, U);
// { dg-error "" "" { target *-*-* } .-1 }

struct Bar<T = [u8; N], const N: usize>(T);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}

