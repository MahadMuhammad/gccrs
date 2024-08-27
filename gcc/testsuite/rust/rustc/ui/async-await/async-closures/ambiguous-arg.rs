// { dg-additional-options "-frust-edition=2021" }

// Regression test for #123901. We previously ICE'd as we silently
// swallowed an in the `ExprUseVisitor`.

#![feature(async_closure)]

pub fn test(test: &u64, temp: &u64) {
    async |check, a, b| {
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        temp.abs_diff(12);
    };
}

fn main() {}

