#![feature(non_lifetime_binders, generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn foo() -> usize
where
    for<T> [i32; { let _: T = todo!(); 0 }]:,
// { dg-error "" "" { target *-*-* } .-1 }
{ 42 }

fn main() {}

