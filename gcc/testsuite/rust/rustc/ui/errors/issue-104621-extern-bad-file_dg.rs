//@ compile-flags: --extern foo={{src-base}}/errors/issue-104621-extern-bad-file.rs
//@ only-linux

extern crate foo;
// { dg-error ".E0463." "" { target *-*-* } .-1 }
// { dg-error ".E0463." "" { target *-*-* } .-2 }
// { dg-error ".E0463." "" { target *-*-* } .-3 }
fn main() {}

