//@ run-pass
//@ aux-build:tokyo.rs
//@ compile-flags:--extern tokyo
// { dg-additional-options "-frust-edition=2021" }

use tokyo::main;

#[main]
fn main() {
    panic!("the #[main] macro should replace this with non-panicking code")
}

