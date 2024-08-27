#![warn(unused_lifetimes, redundant_lifetimes)]

pub trait X {
    type Y<'a: 'static>; // { dg-warning "" "" { target *-*-* } }
}

impl X for () {
    type Y<'a> = &'a ();
// { dg-error ".E0478." "" { target *-*-* } .-1 }
}

struct B<'a, T: for<'r> X<Y<'r> = &'r ()>> {
    f: <T as X>::Y<'a>,
// { dg-error ".E0478." "" { target *-*-* } .-1 }
}

struct C<'a, T: X> {
    f: <T as X>::Y<'a>,
// { dg-error ".E0478." "" { target *-*-* } .-1 }
}

struct D<'a> {
    f: <() as X>::Y<'a>,
// { dg-error ".E0478." "" { target *-*-* } .-1 }
}

fn main() {}

