//@ check-pass

// this test makes sure that the `unfulfilled_lint_expectations` lint
// is being emited for `foo` as foo is not dead code, it's pub

#![warn(dead_code)] // to override compiletest

#[expect(dead_code)]
// { dg-warning "" "" { target *-*-* } .-1 }
pub fn foo() {}

fn main() {}

