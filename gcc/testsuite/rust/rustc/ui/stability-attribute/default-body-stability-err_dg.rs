//@ aux-build:default_body.rs
#![crate_type = "lib"]

extern crate default_body;

use default_body::{Equal, JustTrait};

struct Type;

impl JustTrait for Type {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
// { dg-error ".E0046." "" { target *-*-* } .-3 }

impl Equal for Type {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    fn neq(&self, other: &Self) -> bool {
        false
    }
}

