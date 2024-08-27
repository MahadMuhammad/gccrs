#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    const C: u32 = 0;
    type Type;
    fn bar() {}
    fn foo(&self, x: i32) -> i32 { x }
}

struct F;
impl Trait for F {
    type Type = i32;
}

impl F {
    fn foo(&self, x: i32) -> i32 { x }
}

struct S(F);

impl Trait for S {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    reuse <F as Trait>::C;
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { dg-error ".E0423." "" { target *-*-* } .-2 }
    reuse <F as Trait>::Type;
// { dg-error ".E0575." "" { target *-*-* } .-1 }
// { dg-error ".E0575." "" { target *-*-* } .-2 }
    reuse <F as Trait>::baz;
// { dg-error ".E0576." "" { target *-*-* } .-1 }
// { dg-error ".E0576." "" { target *-*-* } .-2 }
    reuse <F as Trait>::bar;

    reuse foo { &self.0 }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    reuse Trait::foo2 { self.0 }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
}

mod prefix {}
reuse unresolved_prefix::{a, b, c}; // { dg-error ".E0433." "" { target *-*-* } }
reuse prefix::{self, super, crate}; // { dg-error ".E0433." "" { target *-*-* } }

fn main() {}

