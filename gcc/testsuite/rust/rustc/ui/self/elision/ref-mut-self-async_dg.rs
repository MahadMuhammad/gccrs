// { dg-additional-options "-frust-edition=2018" }

#![allow(non_snake_case)]

use std::pin::Pin;

struct Struct { }

impl Struct {
    // Test using `&mut self` sugar:

    async fn ref_self(&mut self, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    // Test using `&mut Self` explicitly:

    async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() { }

