// { dg-additional-options "-frust-edition=2018" }

#![feature(c_variadic)]

async unsafe extern "C" fn multiple_named_lifetimes<'a, 'b>(_: u8, ...) {}
// { dg-error ".E0700." "" { target *-*-* } .-1 }

fn main() {}

