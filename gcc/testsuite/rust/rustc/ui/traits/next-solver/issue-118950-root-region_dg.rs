//@ compile-flags: -Znext-solver
//
// This is a gnarly test but I don't know how to minimize it, frankly.

#![feature(lazy_type_alias)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait ToUnit<'a> {
    type Unit;
}

trait Overlap<T> {}

type Assoc<'a, T> = <*const T as ToUnit<'a>>::Unit;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

impl<T> Overlap<T> for T {}

impl<T> Overlap<for<'a> fn(Assoc<'a, T>)> for T where Missing: Overlap<T> {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }
// { dg-error ".E0119." "" { target *-*-* } .-2 }

fn main() {}

