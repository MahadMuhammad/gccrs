//! This test checks that even if some associated types have
//! `where Self: Sized` bounds, those without still need to be
//! mentioned in trait objects.

trait Foo {
    type Bar
    where
        Self: Sized;
    type Bop;
}

fn foo(_: &dyn Foo) {}
// { dg-error ".E0191." "" { target *-*-* } .-1 }

trait Bar {
    type Bop;
    type Bar
    where
        Self: Sized;
}

fn bar(_: &dyn Bar) {}
// { dg-error ".E0191." "" { target *-*-* } .-1 }

fn main() {}

