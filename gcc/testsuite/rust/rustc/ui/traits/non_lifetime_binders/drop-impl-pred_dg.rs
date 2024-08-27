//@ revisions: no yes
//@[yes] check-pass

// Issue 110557

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub trait Foo {}

#[cfg(no)]
struct Bar<T>(T) where T: Foo;

#[cfg(yes)]
struct Bar<T>(T) where for<H> H: Foo;

impl<T> Drop for Bar<T>
where
    for<H> H: Foo,
// { dg-error "" "" { target *-*-* } .-1 }
{
    fn drop(&mut self) {}
}

fn main() {}

