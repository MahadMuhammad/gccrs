//@ revisions: feature gated

#![cfg_attr(feature, feature(lazy_type_alias))]
#![allow(incomplete_features)]

type X = Vec<X>;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

#[rustfmt::skip]
fn main() { let b: X = Vec::new(); }

