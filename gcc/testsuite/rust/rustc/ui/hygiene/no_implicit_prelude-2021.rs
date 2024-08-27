//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

#![no_implicit_prelude]

fn main() {
    assert!(true, "hoi");
    assert!(false, "hoi {}", 123);
}

