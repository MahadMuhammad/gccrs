// Regression test for #86667, where a garbled suggestion was issued for
// a missing named lifetime parameter.

//@ compile-flags: --edition 2018

async fn a(s1: &str, s2: &str) -> &str {
// { dg-error ".E0106." "" { target *-*-* } .-1 }
    s1
}

fn b(s1: &str, s2: &str) -> &str {
// { dg-error ".E0106." "" { target *-*-* } .-1 }
    s1
}

fn main() {}

