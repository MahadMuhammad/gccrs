//@ [full] check-pass
//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

struct Foo<const V: [usize; 0] > {}
// { dg-error "" "" { target *-*-* } .-1 }

type MyFoo = Foo<{ [] }>;

fn main() {
    let _ = Foo::<{ [] }> {};
}

