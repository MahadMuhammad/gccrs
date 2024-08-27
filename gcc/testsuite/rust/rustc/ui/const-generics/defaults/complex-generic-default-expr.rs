//@ revisions: full min
//@[full] check-pass
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

struct Foo<const N: usize, const M: usize = { N + 1 }>;
// { dg-error "" "" { target *-*-* } .-1 }

struct Bar<T, const TYPE_SIZE: usize = { std::mem::size_of::<T>() }>(T);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

