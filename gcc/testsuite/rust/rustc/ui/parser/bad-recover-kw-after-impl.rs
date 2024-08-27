// This is just `mbe-async-trait-bound-theoretical-regression.rs` in practice.

// { dg-additional-options "-frust-edition=2021" }
// for the `impl` + keyword test

macro_rules! impl_primitive {
    ($ty:ty) => {
        compile_error!("whoops");
    };
    (impl async) => {};
}

impl_primitive!(impl async);
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

fn main() {}

