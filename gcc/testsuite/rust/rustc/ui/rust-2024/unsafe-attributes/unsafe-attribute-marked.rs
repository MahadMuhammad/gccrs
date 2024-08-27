//@ revisions: edition2021 edition2024
// { dg-additional-options "-frust-edition=2021" }
// { dg-additional-options "-frust-edition=2024" }
//@[edition2024] compile-flags: -Zunstable-options
//@ check-pass


#[unsafe(no_mangle)]
extern "C" fn foo() {}

fn main() {}

