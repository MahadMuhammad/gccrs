#![deny(deref_into_dyn_supertrait)]

use std::ops::Deref;

trait Bar<'a> {}
trait Foo<'a>: Bar<'a> {}

impl<'a> Deref for dyn Foo<'a> {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    type Target = dyn Bar<'a>;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

fn main() {}

