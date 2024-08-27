// Regression test for #73727

//@ revisions: full min
//@[full]check-pass

#![cfg_attr(full, feature(adt_const_params, unsized_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

fn a<const X: &'static [u32]>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    a::<{ &[] }>();
}

