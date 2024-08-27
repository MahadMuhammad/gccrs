// Check what happens when `fun` is used to define a function, instead of `fn`
// { dg-additional-options "-frust-edition=2021" }

#![allow(dead_code)]

fun foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}

