//@ compile-flags: -Znext-solver

use std::collections::HashMap;

fn foo() -> &'static HashMap<i32, i32>
{
    &HashMap::new()
// { dg-error ".E0515." "" { target *-*-* } .-1 }
}

fn main() {}

