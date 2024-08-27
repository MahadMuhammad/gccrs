#![deny(unknown_or_malformed_diagnostic_attributes)]

#[diagnostic::unknown_attribute]
// { dg-error "" "" { target *-*-* } .-1 }
struct Foo;

fn main() {}

