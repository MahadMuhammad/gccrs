//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params, unsized_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

trait A {}
struct B;
impl A for B {}

fn test<const T: &'static dyn A>() {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    unimplemented!()
}

fn main() {
    test::<{ &B }>();
}

