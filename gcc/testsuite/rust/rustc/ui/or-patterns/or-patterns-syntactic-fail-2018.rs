// Test that :pat doesn't accept top-level or-patterns in edition 2018.

// { dg-additional-options "-frust-edition=2018" }

fn main() {}

// Test the `pat` macro fragment parser:
macro_rules! accept_pat {
    ($p:pat) => {};
}

accept_pat!(p | q); // { dg-error "" "" { target *-*-* } }
accept_pat!(|p| q); // { dg-error "" "" { target *-*-* } }

