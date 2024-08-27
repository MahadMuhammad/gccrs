//@ aux-build:unsized_const_param.rs
#![feature(adt_const_params, unsized_const_params)]
// { dg-warning "" "" { target *-*-* } .-1 }

extern crate unsized_const_param;

#[derive(std::marker::ConstParamTy, Eq, PartialEq)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
struct A(unsized_const_param::GenericNotUnsizedParam<&'static [u8]>);

#[derive(std::marker::UnsizedConstParamTy, Eq, PartialEq)]
struct B(unsized_const_param::GenericNotUnsizedParam<&'static [u8]>);

fn main() {}

