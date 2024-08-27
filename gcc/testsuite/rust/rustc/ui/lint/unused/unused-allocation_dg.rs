#![feature(rustc_attrs, stmt_expr_attributes)]
#![deny(unused_allocation)]

fn main() {
    _ = (#[rustc_box] Box::new([1])).len(); // { dg-error "" "" { target *-*-* } }
    _ = Box::new([1]).len(); // { dg-error "" "" { target *-*-* } }
}

