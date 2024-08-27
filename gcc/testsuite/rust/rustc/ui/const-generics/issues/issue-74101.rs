//@ [full] check-pass
//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

fn test<const N: [u8; 1 + 2]>() {}
// { dg-error "" "" { target *-*-* } .-1 }

struct Foo<const N: [u8; 1 + 2]>;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

