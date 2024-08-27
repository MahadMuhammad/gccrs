//@ [full] check-pass
//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

const LEN: usize = 1024;

fn hoge<const IN: [u32; LEN]>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

