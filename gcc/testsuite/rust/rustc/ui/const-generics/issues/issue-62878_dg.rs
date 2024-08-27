//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params, generic_arg_infer))]
#![cfg_attr(full, allow(incomplete_features))]

fn foo<const N: usize, const A: [u8; N]>() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {
    foo::<_, { [1] }>();
// { dg-error "" "" { target *-*-* } .-1 }
}

