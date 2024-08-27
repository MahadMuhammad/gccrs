// Check that `unused_lifetimes` lint doesn't duplicate a "parameter is never used" error.
// Fixed in <https://github.com/rust-lang/rust/pull/96833>.
// Issue: <https://github.com/rust-lang/rust/issues/72587>.

#![warn(unused_lifetimes)]
struct Foo<'a>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }

fn main() {}

