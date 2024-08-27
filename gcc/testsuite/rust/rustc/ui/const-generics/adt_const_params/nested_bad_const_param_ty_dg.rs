#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

#[derive(ConstParamTy)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
// { dg-error ".E0204." "" { target *-*-* } .-2 }
struct Foo([*const u8; 1]);

#[derive(ConstParamTy)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
// { dg-error ".E0204." "" { target *-*-* } .-2 }
struct Foo2([*mut u8; 1]);

#[derive(ConstParamTy)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
// { dg-error ".E0204." "" { target *-*-* } .-2 }
struct Foo3([fn(); 1]);

fn main() {}

