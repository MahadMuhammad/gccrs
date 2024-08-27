//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

// https://github.com/rust-lang/rust/issues/123573#issue-2229428739

#![warn(non_local_definitions)]

pub trait Test {}

impl<'a, T: 'a> Test for &[T] where &'a T: Test {}

fn main() {
    struct Local {}
    impl Test for &Local {}
// { dg-warning "" "" { target *-*-* } .-1 }
}

