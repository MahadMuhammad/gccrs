//@ revisions: old next
//@[next] compile-flags: -Znext-solver

struct Foo<const N: u8 = { 255 + 1 }>;
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {}

