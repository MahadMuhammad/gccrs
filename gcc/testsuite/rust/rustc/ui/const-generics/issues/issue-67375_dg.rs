//@ revisions: full min
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, feature(generic_const_exprs))]

struct Bug<T> {
// { dg-error "" "" { target *-*-* } .-1 }
    inner: [(); { [|_: &T| {}; 0].len() }],
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

