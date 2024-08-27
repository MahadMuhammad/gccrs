// Checks what happens when `public` is used instead of the correct, `pub`
// { dg-additional-options "-frust-edition=2018" }
//@ run-rustfix
public struct X;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}

