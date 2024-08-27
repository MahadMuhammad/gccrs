//@ revisions: default generic_const_items

#![cfg_attr(generic_const_items, feature(generic_const_items), allow(incomplete_features))]

trait ZstAssert: Sized {
    const A: &str = "";
    const B: S = S { s: &() }; // { dg-error "" "" { target *-*-* } }
    const C: &'_ str = "";
    const D: T = T { a: &(), b: &() }; // { dg-error "" "" { target *-*-* } }
}

struct S<'a> {
    s: &'a (),
}
struct T<'a, 'b> {
    a: &'a (),
    b: &'b (),
}

fn main() {}

