// { dg-additional-options "-frust-edition=2018" }

// Regression test for #95307.
// The ICE occurred on all the editions, specifying edition:2018 to reduce diagnostics.

pub trait C {
    async fn new() -> [u8; _];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
}

fn main() {}

