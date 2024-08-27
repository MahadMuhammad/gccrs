#![feature(adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use std::marker::{ConstParamTy_, UnsizedConstParamTy};

fn foo(a: &dyn ConstParamTy_) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }

fn bar(a: &dyn UnsizedConstParamTy) {}
// { dg-error ".E0038." "" { target *-*-* } .-1 }

fn main() {}

