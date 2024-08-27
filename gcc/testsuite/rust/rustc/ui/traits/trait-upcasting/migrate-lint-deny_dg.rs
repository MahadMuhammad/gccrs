#![deny(deref_into_dyn_supertrait)]

use std::ops::Deref;

// issue 89190
trait A {}
trait B: A {}

impl<'a> Deref for dyn 'a + B {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    type Target = dyn A;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

fn take_a(_: &dyn A) {}

fn whoops(b: &dyn B) {
    take_a(b)
}

fn main() {}

