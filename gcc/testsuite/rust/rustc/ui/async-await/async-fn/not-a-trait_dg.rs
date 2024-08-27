// { dg-additional-options "-frust-edition=2018" }

#![feature(async_closure)]

struct S;

fn test(x: impl async S) {}
// { dg-error ".E0404." "" { target *-*-* } .-1 }

fn missing(x: impl async Missing) {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }

fn main() {}

