// { dg-additional-options "-frust-edition=2018" }
//@ revisions: full min

#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

struct Foo;

impl<const H: feature> Foo {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    async fn biz() {}
}

fn main() {}

