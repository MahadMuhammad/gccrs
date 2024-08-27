#![deny(unused_attributes)]
#![allow()] // { dg-error "" "" { target *-*-* } }
#![expect()] // { dg-error "" "" { target *-*-* } }
#![warn()] // { dg-error "" "" { target *-*-* } }
#![deny()] // { dg-error "" "" { target *-*-* } }
#![forbid()] // { dg-error "" "" { target *-*-* } }
#![feature()] // { dg-error "" "" { target *-*-* } }

#[repr()] // { dg-error "" "" { target *-*-* } }
pub struct S;

#[target_feature()] // { dg-error "" "" { target *-*-* } }
pub unsafe fn foo() {}

fn main() {}

