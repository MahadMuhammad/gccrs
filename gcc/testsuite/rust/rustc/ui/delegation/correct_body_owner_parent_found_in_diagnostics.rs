#![feature(fn_delegation)]
#![allow(incomplete_features)]

use std::marker::PhantomData;

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(&());
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

trait Trait {
    fn foo(&self) -> u8 { 0 }
    fn bar(&self) -> u8 { 1 }
    fn meh(&self) -> u8 { 2 }
}

struct Z(u8);

impl Trait for Z {
    reuse <u8 as Trait>::{foo, bar, meh} { &const { InvariantRef::<'a>::NEW } }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
// { dg-error ".E0277." "" { target *-*-* } .-5 }
// { dg-error ".E0277." "" { target *-*-* } .-6 }
// { dg-error ".E0277." "" { target *-*-* } .-7 }
// { dg-error ".E0277." "" { target *-*-* } .-8 }
// { dg-error ".E0277." "" { target *-*-* } .-9 }
}

fn main() { }

