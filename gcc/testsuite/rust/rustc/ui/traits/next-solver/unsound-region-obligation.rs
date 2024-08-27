// { dg-error "" "" { target *-*-* } }
//@ compile-flags: -Znext-solver
// Regression test for rust-lang/trait-system-refactor-initiative#59

trait StaticTy {
    type Item<'a>: 'static;
}

impl StaticTy for () {
    type Item<'a> = &'a ();
}

fn main() {}

