// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

fn main() {
    fn needs_fn(x: impl FnOnce()) {}
    needs_fn(async || {});
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

