#![feature(adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use std::marker::UnsizedConstParamTy;

struct Foo;

impl UnsizedConstParamTy for &'static Foo {}
// { dg-error ".E0204." "" { target *-*-* } .-1 }

fn main() {}

