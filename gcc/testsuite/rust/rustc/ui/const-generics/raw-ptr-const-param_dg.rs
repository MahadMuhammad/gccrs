//@ revisions: min adt_const_params full

#![cfg_attr(full, feature(adt_const_params, unsized_const_params))]
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(adt_const_params, feature(adt_const_params))]
#![cfg_attr(adt_const_params, allow(incomplete_features))]

struct Const<const P: *const u32>; // { dg-error "" "" { target *-*-* } }

fn main() {
    let _: Const<{ 15 as *const _ }> = Const::<{ 10 as *const _ }>;
// { dg-error "" "" { target *-*-* } .-1 }
    let _: Const<{ 10 as *const _ }> = Const::<{ 10 as *const _ }>;
}

