#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]

#[derive(PartialEq, Eq)]
struct NotParam;

fn check<T: std::marker::UnsizedConstParamTy + ?Sized>() {}

fn main() {
    check::<&NotParam>(); // { dg-error ".E0277." "" { target *-*-* } }
    check::<[NotParam]>(); // { dg-error ".E0277." "" { target *-*-* } }
    check::<[NotParam; 17]>(); // { dg-error ".E0277." "" { target *-*-* } }
}

