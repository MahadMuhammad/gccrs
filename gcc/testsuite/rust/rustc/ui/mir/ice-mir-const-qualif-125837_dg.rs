// Test for ICE: mir_const_qualif: index out of bounds: the len is 0 but the index is 0
// https://github.com/rust-lang/rust/issues/125837

use std::fmt::Debug;

trait Foo<Item> {}

impl<Item, D: Debug + Clone> Foo for D {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    fn foo<'a>(&'a self) -> impl Debug {
// { dg-error ".E0407." "" { target *-*-* } .-1 }
        const { return }
// { dg-error ".E0572." "" { target *-*-* } .-1 }
    }
}

pub fn main() {}

