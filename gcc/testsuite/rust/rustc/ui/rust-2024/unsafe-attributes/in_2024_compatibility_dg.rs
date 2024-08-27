#![deny(rust_2024_compatibility)]

#[no_mangle]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
extern "C" fn foo() {}

fn main() {}

