//@ revisions: feature gated

#![cfg_attr(feature, feature(lazy_type_alias))]
#![allow(incomplete_features)]

type X1 = X2;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
type X2 = X3;
// { dg-error "" "" { target *-*-* } .-1 }
type X3 = X1;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

