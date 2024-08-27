//@ aux-build:empty.rs
//@ revisions: normal exhaustive_patterns
//
// This tests a match with no arms on various types, and checks NOTEs.
#![feature(never_type)]
#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![deny(unreachable_patterns)]
// { dg-note "" "" { target *-*-* } .-1 }

extern crate empty;

enum EmptyEnum {}

fn empty_enum(x: EmptyEnum) {
    match x {} // ok
    match x {
        _ => {} // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    }
    match x {
        _ if false => {} // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    }
}

fn empty_foreign_enum(x: empty::EmptyForeignEnum) {
    match x {} // ok
    match x {
        _ => {} // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    }
    match x {
        _ if false => {} // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    }
}

fn empty_foreign_enum_private(x: &Option<empty::SecretlyUninhabitedForeignStruct>) {
    let None = *x;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
}

fn main() {
    match 0u8 {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
        _ if false => {}
    }
}

