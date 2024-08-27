#![feature(adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

use std::marker::UnsizedConstParamTy;

#[derive(Eq, PartialEq)]
struct Foo<T>(T);

trait Other {}

impl<T> UnsizedConstParamTy for Foo<T> where T: Other + UnsizedConstParamTy {}

fn foo<const N: Foo<u8>>() {}
// { dg-error ".E0741." "" { target *-*-* } .-1 }
// { dg-note ".E0741." "" { target *-*-* } .-2 }

fn main() {}

