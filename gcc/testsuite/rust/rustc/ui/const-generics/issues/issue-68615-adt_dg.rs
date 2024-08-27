//@ [full] check-pass
//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

struct Const<const V: [usize; 0]> {}
// { dg-error "" "" { target *-*-* } .-1 }
type MyConst = Const<{ [] }>;

fn main() {
    let _x = Const::<{ [] }> {};
    let _y = MyConst {};
}

