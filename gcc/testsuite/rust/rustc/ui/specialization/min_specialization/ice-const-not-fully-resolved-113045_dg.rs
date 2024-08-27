// ICE min_specialization:
// Ok(['?0, Const { ty: usize, kind: Leaf(0x0000000000000000) }]) is not fully resolved
// issue: rust-lang/rust#113045

#![feature(min_specialization)]

trait X {}

impl<'a, const N: usize> X for [(); N] {}

impl<'a, Unconstrained> X for [(); 0] {}
// { dg-error ".E0207." "" { target *-*-* } .-1 }
// { dg-error ".E0207." "" { target *-*-* } .-2 }

fn main() {}

