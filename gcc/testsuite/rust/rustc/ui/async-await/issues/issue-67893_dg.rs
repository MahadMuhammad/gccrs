//@ aux-build: issue_67893.rs
// { dg-additional-options "-frust-edition=2018" }

extern crate issue_67893;

fn g(_: impl Send) {}

fn main() {
    g(issue_67893::run())
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

