// Regression test for #66975
#![feature(never_type)]

const VOID: ! = panic!();
// { dg-error ".E0080." "" { target *-*-* } .-1 }

fn main() {
    let _ = VOID;
}

