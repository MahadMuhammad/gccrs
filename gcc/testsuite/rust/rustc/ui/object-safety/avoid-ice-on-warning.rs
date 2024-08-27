//@ revisions: old new
// { dg-additional-options "-frust-edition=2015" }
// { dg-additional-options "-frust-edition=2021" }
fn call_this<F>(f: F) : Fn(&str) + call_that {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
fn main() {}

