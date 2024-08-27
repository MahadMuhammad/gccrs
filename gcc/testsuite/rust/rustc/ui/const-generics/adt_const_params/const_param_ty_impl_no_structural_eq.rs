#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]

#[derive(PartialEq, Eq)]
struct ImplementsConstParamTy;
impl std::marker::UnsizedConstParamTy for ImplementsConstParamTy {}

struct CantParam(ImplementsConstParamTy);

impl std::marker::UnsizedConstParamTy for CantParam {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

#[derive(std::marker::UnsizedConstParamTy)]
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
struct CantParamDerive(ImplementsConstParamTy);

fn check<T: std::marker::UnsizedConstParamTy>() {}

fn main() {
    check::<ImplementsConstParamTy>();
}

