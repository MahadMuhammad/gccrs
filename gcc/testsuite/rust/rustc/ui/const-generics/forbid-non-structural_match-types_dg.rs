#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy)]
struct A;

struct B<const X: A>; // ok

struct C;

struct D<const X: C>; // { dg-error ".E0741." "" { target *-*-* } }

fn main() {}

