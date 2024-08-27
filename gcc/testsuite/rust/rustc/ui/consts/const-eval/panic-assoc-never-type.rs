//@ build-fail

// Regression test for #66975
#![feature(never_type)]

struct PrintName;

impl PrintName {
    const VOID: ! = panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }
}

fn main() {
    let _ = PrintName::VOID; // { dg-error "" "" { target *-*-* } }
}

