// { dg-additional-options "-frust-edition=2018" }
//@ run-rustfix
#![allow(non_snake_case, dead_code)]

use std::pin::Pin;

struct Struct<'a> {
    data: &'a u32,
}

impl<'a> Struct<'a> {
    // Test using `&self` sugar:

    async fn ref_self(&self, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    // Test using `&Self` explicitly:

    async fn ref_Self(self: &Self, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

