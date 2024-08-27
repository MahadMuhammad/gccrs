//@ compile-flags: -Z deduplicate-diagnostics=yes

#![warn(unused_imports)]

use std::str::*;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }

mod hey {
    pub trait Serialize {}
    pub trait Deserialize {}

    pub struct X(i32);
}

use hey::{Serialize, Deserialize, X};
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }

#[derive(Serialize)]
// { dg-error "" "" { target *-*-* } .-1 }
struct A;

#[derive(from_utf8_mut)]
// { dg-error "" "" { target *-*-* } .-1 }
struct B;

#[derive(println)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
struct C;

#[Deserialize]
// { dg-error "" "" { target *-*-* } .-1 }
struct D;

#[from_utf8_unchecked]
// { dg-error "" "" { target *-*-* } .-1 }
struct E;

#[println]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
struct F;

fn main() {
    from_utf8!();
// { dg-error "" "" { target *-*-* } .-1 }

    Box!();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

    Copy!();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

    test!();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

    X!();
// { dg-error "" "" { target *-*-* } .-1 }
}

