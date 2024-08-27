//@ compile-flags: -Znext-solver
// { dg-error "" "" { target *-*-* } .-1 }
#![feature(specialization)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Default {
    type Id;

    fn intu(&self) -> &Self::Id;
}

impl<T> Default for T {
    default type Id = T;
    // This will be fixed by #111994
    fn intu(&self) -> &Self::Id {
// { dg-error ".E0284." "" { target *-*-* } .-1 }
        self // { dg-error ".E0284." "" { target *-*-* } }
    }
}

fn transmute<T: Default<Id = U>, U: Copy>(t: T) -> U {
    *t.intu()
}

use std::num::NonZero;

fn main() {
    let s = transmute::<u8, Option<NonZero<u8>>>(0); // { dg-error ".E0284." "" { target *-*-* } }
    assert_eq!(s, None);
}

