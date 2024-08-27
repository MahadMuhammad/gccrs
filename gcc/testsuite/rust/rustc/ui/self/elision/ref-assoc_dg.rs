#![allow(non_snake_case)]

use std::pin::Pin;

trait Trait {
    type AssocType;
}

struct Struct { }

impl Trait for Struct {
    type AssocType = Self;
}

impl Struct {
    fn ref_AssocType(self: &<Struct as Trait>::AssocType, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    fn box_ref_AssocType(self: Box<&<Struct as Trait>::AssocType>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    fn pin_ref_AssocType(self: Pin<&<Struct as Trait>::AssocType>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    fn box_box_ref_AssocType(self: Box<Box<&<Struct as Trait>::AssocType>>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }

    fn box_pin_ref_AssocType(self: Box<Pin<&<Struct as Trait>::AssocType>>, f: &u32) -> &u32 {
        f
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() { }

