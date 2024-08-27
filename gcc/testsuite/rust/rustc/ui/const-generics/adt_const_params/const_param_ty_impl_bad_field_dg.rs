#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]

#[derive(PartialEq, Eq)]
struct NotParam;

#[derive(PartialEq, Eq)]
struct CantParam(NotParam);

impl std::marker::UnsizedConstParamTy for CantParam {}
// { dg-error ".E0204." "" { target *-*-* } .-1 }

#[derive(std::marker::UnsizedConstParamTy, Eq, PartialEq)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
struct CantParamDerive(NotParam);

fn main() {}

