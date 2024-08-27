// This ensures we don't ICE in situations like rust-lang/rust#126272.

#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

#[derive(Debug, PartialEq, Eq, ConstParamTy)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
// { dg-error ".E0204." "" { target *-*-* } .-2 }
struct Foo {
    nested: &'static Bar<dyn std::fmt::Debug>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
// { dg-error ".E0277." "" { target *-*-* } .-5 }
}

#[derive(Debug, PartialEq, Eq, ConstParamTy)]
struct Bar<T>(T);

struct Test<const F: Foo>;

fn main() {
    let x: Test<{ Foo { nested: &Bar(4) } }> = Test;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

