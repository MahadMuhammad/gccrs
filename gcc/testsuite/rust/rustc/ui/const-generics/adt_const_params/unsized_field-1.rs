//@ aux-build:unsized_const_param.rs
#![feature(adt_const_params)]

extern crate unsized_const_param;

use std::marker::ConstParamTy;

#[derive(ConstParamTy, Eq, PartialEq)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
struct A([u8]);

#[derive(ConstParamTy, Eq, PartialEq)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
struct B(&'static [u8]);

#[derive(ConstParamTy, Eq, PartialEq)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
struct C(unsized_const_param::Foo);

fn main() {}

