#![feature(fn_delegation)]
#![allow(incomplete_features)]

trait Trait {
    fn method(&self);
    const CONST: u8;
    type Type;
    #[allow(non_camel_case_types)]
    type method;
}

impl Trait for u8 {
    fn method(&self) {}
    const CONST: u8 = 0;
    type Type = u8;
    type method = u8;
}

struct Good(u8);
impl Trait for Good {
    reuse Trait::* { &self.0 }
    // Explicit definitions for non-delegatable items.
    const CONST: u8 = 0;
    type Type = u8;
    type method = u8;
}

struct Bad(u8);
impl Trait for Bad { // { dg-error ".E0046." "" { target *-*-* } }
    reuse Trait::* { &self.0 }
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { dg-error ".E0423." "" { target *-*-* } .-2 }
// { dg-error ".E0423." "" { target *-*-* } .-3 }
// { dg-error ".E0423." "" { target *-*-* } .-4 }
// { dg-error ".E0423." "" { target *-*-* } .-5 }
}

fn main() {}

