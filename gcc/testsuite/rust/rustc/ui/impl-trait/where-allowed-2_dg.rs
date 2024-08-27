use std::fmt::Debug;

fn in_adt_in_return() -> Vec<impl Debug> { panic!() }
// { dg-error ".E0283." "" { target *-*-* } .-1 }

fn main() {}

