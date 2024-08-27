//@ revisions: edition2021 edition2024
// { dg-additional-options "-frust-edition=2021" }
//@[edition2021] check-pass
// { dg-additional-options "-frust-edition=2024" }
//@[edition2024] compile-flags: -Zunstable-options


#[no_mangle] // { dg-error "" "" { target *-*-* } }
extern "C" fn foo() {}

fn main() {}

