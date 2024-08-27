// Invariant checking doesn't ICE in some cases with errors (issue #104249).

#![feature(staged_api)] // { dg-error "" "" { target *-*-* } }

pub mod m {} // { dg-error "" "" { target *-*-* } }

pub mod m { // { dg-error ".E0428." "" { target *-*-* } }
    mod inner {}
    type Inner = u8;
}

fn main() {}

