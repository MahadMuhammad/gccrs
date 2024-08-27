// { dg-additional-options "-frust-edition=2018" }
#![feature(must_not_suspend)]
#![deny(must_not_suspend)]

#[must_not_suspend] // { dg-error "" "" { target *-*-* } }
fn foo() -> i32 {
    0
}
fn main() {}

