//@ revisions: e2024 none
//@[e2024] compile-flags: --edition 2024 -Zunstable-options

gen fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}

