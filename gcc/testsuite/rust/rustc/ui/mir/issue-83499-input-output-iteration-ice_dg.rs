// Test that when in MIR the amount of local_decls and amount of normalized_input_tys don't match
// that an out-of-bounds access does not occur.
#![feature(c_variadic)]

fn main() {}

fn foo(_: Bar, ...) -> impl {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
// { dg-error ".E0412." "" { target *-*-* } .-3 }

