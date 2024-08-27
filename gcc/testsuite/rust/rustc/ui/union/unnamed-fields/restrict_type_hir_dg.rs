//@ aux-build:dep.rs

// test for #121151

#![allow(incomplete_features)]
#![feature(unnamed_fields)]

extern crate dep;

#[repr(C)]
struct A {
    a: u8,
}

enum BadEnum {
    A,
    B,
}

#[repr(C)]
enum BadEnum2 {
    A,
    B,
}

type MyStruct = A;
type MyI32 = i32;

#[repr(C)]
struct L {
    _: i32, // { dg-error "" "" { target *-*-* } }
    _: MyI32, // { dg-error "" "" { target *-*-* } }
    _: BadEnum, // { dg-error "" "" { target *-*-* } }
    _: BadEnum2, // { dg-error "" "" { target *-*-* } }
    _: MyStruct,
    _: dep::BadStruct, // { dg-error "" "" { target *-*-* } }
    _: dep::BadEnum, // { dg-error "" "" { target *-*-* } }
    _: dep::BadEnum2, // { dg-error "" "" { target *-*-* } }
    _: dep::BadAlias, // { dg-error "" "" { target *-*-* } }
    _: dep::GoodAlias,
    _: dep::GoodStruct,
}

fn main() {}

