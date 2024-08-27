//@ aux-build:extern-crate.rs
extern crate extern_crate;

impl extern_crate::StructWithAttr {}
// { dg-error ".E0116." "" { target *-*-* } .-1 }

impl extern_crate::StructNoAttr {}
// { dg-error ".E0116." "" { target *-*-* } .-1 }

impl extern_crate::EnumWithAttr {}
// { dg-error ".E0116." "" { target *-*-* } .-1 }

impl extern_crate::EnumNoAttr {}
// { dg-error ".E0116." "" { target *-*-* } .-1 }

impl f32 {} // { dg-error ".E0390." "" { target *-*-* } }

fn main() {}

