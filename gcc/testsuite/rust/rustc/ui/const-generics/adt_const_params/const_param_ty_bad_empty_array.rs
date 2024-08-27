#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]

#[derive(PartialEq, Eq)]
struct NotParam;

fn check<T: std::marker::ConstParamTy_>() {}

fn main() {
    check::<[NotParam; 0]>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

