//@ compile-flags: --target x86_64-unknown-uefi
//@ needs-llvm-components: x86
//@ rustc-env:CARGO_CRATE_NAME=foo
#![feature(no_core)]
#![no_core]
extern crate core;
// { dg-error ".E0463." "" { target *-*-* } .-1 }
// { dg-note ".E0463." "" { target *-*-* } .-2 }
// { dg-note ".E0463." "" { target *-*-* } .-3 }
// { help ".E0463." "" { target *-*-* } .-4 }
// { help ".E0463." "" { target *-*-* } .-5 }
fn main() {}

