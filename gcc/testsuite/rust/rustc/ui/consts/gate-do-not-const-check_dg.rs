#[rustc_do_not_const_check]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
const fn foo() {}

fn main() {}

